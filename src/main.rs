use devices::{builtin::cpal::CpalProvider, traits::Device, traits::DeviceProvider};
use media::{
    builtin::symphonia::SymphoniaProvider,
    traits::{MediaProvider, MetadataProvider},
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

    println!("{:?}", metadata);
    println!("opening device");

    let mut dev_provider = CpalProvider::default();
    let mut device = dev_provider
        .get_default_device()
        .expect("no default device");
    let format = device.get_default_format().expect("no default format");
    device.open_device(format).expect("unable to open device");

    println!("device should be open");
}
