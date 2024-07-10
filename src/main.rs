use std::io::{Read, Write};

use devices::{
    builtin::cpal::CpalProvider,
    resample::Convertable,
    traits::{Device, DeviceProvider},
};
use media::{
    builtin::symphonia::SymphoniaProvider,
    traits::{MediaProvider, MetadataProvider, PlaybackProvider},
};

mod devices;
mod media;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("file path not provided");

    let src = std::fs::File::open(&path).expect("failed to open media");

    let mut provider = SymphoniaProvider::default();

    provider.open(src, None).expect("unsupported format?");

    let metadata = provider
        .get_metadata_provider()
        .unwrap()
        .read_metadata()
        .expect("unknown error");

    println!("metadata: {:?}", metadata);
    println!("opening device");

    let mut dev_provider = CpalProvider::default();
    let mut device = dev_provider
        .get_default_device()
        .expect("no default device");
    let format = device.get_default_format().expect("no default format");
    let mut stream = device.open_device(format).expect("unable to open device");
    let device_format = stream
        .get_current_format()
        .expect("device should be open")
        .clone();

    println!("device name: {:?}", device.get_name());
    println!("device information: {:?}", device_format);

    println!("starting decode");
    provider.start_playback().expect("unable to start decode");
    println!("HOPEFULLY AUDIO PLAYS");

    loop {
        let first_samples = provider.read_samples().expect("unable to read samples");
        let converted = first_samples.convert_formats(device_format.clone());
        stream.submit_frame(converted);
    }
}
