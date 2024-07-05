use providers::{
    builtin::symphonia::SymphoniaProvider,
    traits::{MetadataProvider, PlaybackProvider, Provider},
};

mod providers;

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
}
