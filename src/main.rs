mod devices;
mod media;
mod playback;
mod ui;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("cannot set subscriber");

    tracing::info!("Starting application");
    crate::ui::app::run();
}
