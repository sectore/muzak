use std::{
    convert,
    io::{Read, Write},
    thread::sleep,
    time::Duration,
};

use devices::{
    builtin::cpal::CpalProvider,
    format::ChannelSpec,
    resample::Resampler,
    traits::{Device, DeviceProvider},
};
use media::{
    builtin::symphonia::SymphoniaProvider,
    playback::UnwrapSample,
    traits::{MediaProvider, MetadataProvider, PlaybackProvider},
};
use symphonia::core::{conv, sample};

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

    let first_samples = provider.read_samples().expect("unable to read samples");
    let duration = provider.duration_frames().expect("can't get duration");
    println!("frame duration: {:?}", duration);
    let mut resampler = Resampler::new(
        first_samples.rate,
        device_format.sample_rate,
        duration,
        match device_format.channels {
            ChannelSpec::Count(v) => v,
            _ => 2,
        },
    );
    let converted = resampler.convert_formats(first_samples, device_format.clone());
    stream.submit_frame(converted);

    loop {
        let samples = match provider.read_samples() {
            Ok(v) => v,
            Err(err) => match err {
                media::errors::PlaybackReadError::NothingOpen => panic!("nothing open"),
                media::errors::PlaybackReadError::NeverStarted => panic!("playback never started"),
                media::errors::PlaybackReadError::EOF => break,
                media::errors::PlaybackReadError::Unknown => panic!("unknown error"),
                media::errors::PlaybackReadError::DecodeFatal => panic!("fatal decode error"),
            },
        };

        let converted = resampler.convert_formats(samples, device_format.clone());
        stream.submit_frame(converted);
    }

    println!("end of file reached, waiting for 1 second");

    sleep(Duration::from_secs(1));
}
