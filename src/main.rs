use gpui::actions;

mod data;
mod devices;
mod media;
mod playback;
mod ui;
mod util;

fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting application");
    crate::ui::app::run();
}
