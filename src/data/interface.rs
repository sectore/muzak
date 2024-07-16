use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use gpui::{AppContext, Model};
use image::RgbaImage;

use crate::ui::models::Models;

use super::events::{DataCommand, DataEvent, ImageLayout, ImageType};

/// The DataInterface trait defines the method used to create the struct that will be used to
/// communicate between the data thread and the main thread.
pub trait DataInterface {
    fn new(commands_tx: Sender<DataCommand>, events_rx: Receiver<DataEvent>) -> Self;
}

pub struct GPUIDataInterface {
    commands_tx: Sender<DataCommand>,
    events_rx: Option<Receiver<DataEvent>>,
}

impl gpui::Global for GPUIDataInterface {}

/// The data interface struct that will be used to communicate between the data thread and the main
/// thread. This implementation takes advantage of the GPUI Global trait to allow any function (so
/// long as it is running on the main thread) to send commands to the data thread.
///
/// This interface takes advantage of GPUI's asynchronous runtime to read messages without blocking
/// rendering. Messages are read at quickest every 50ms, however the runtime may choose to run the
/// function that reads events less frequently, depending on the current workload. Because of this,
/// event handling should not perform any heavy operations, which should be added to the data
/// thread.
impl DataInterface for GPUIDataInterface {
    fn new(commands_tx: Sender<DataCommand>, events_rx: Receiver<DataEvent>) -> Self {
        Self {
            commands_tx,
            events_rx: Some(events_rx),
        }
    }
}

impl GPUIDataInterface {
    pub fn decode_image(&self, data: Box<[u8]>, image_type: ImageType, image_layout: ImageLayout) {
        self.commands_tx
            .send(DataCommand::DecodeImage(data, image_type, image_layout))
            .expect("could not send tx");
    }

    /// Starts the broadcast loop that will read events from the data thread and update data models
    /// accordingly. This function should be called once, and will panic if called more than once.
    pub fn start_broadcast(&mut self, cx: &mut AppContext) {
        let mut events_rx = None;
        std::mem::swap(&mut self.events_rx, &mut events_rx);

        let albumart_model: Model<Option<RgbaImage>> = cx.global::<Models>().albumart.clone();

        if let Some(events_rx) = events_rx {
            cx.spawn(|mut cx| async move {
                loop {
                    if let Ok(event) = events_rx.try_recv() {
                        match event {
                            DataEvent::ImageDecoded(v, image_type) => match image_type {
                                ImageType::CurrentAlbumArt => {
                                    albumart_model
                                        .update(&mut cx, |m, cx| {
                                            *m = Some(v);
                                            cx.notify()
                                        })
                                        .expect("failed to update albumart");
                                }
                                _ => todo!(),
                            },
                            DataEvent::DecodeError(image_type) => match image_type {
                                ImageType::CurrentAlbumArt => {
                                    albumart_model
                                        .update(&mut cx, |m, cx| {
                                            *m = None;
                                            cx.notify()
                                        })
                                        .expect("failed to update albumart");
                                }
                                _ => todo!(),
                            },
                            _ => (),
                        }
                    }

                    cx.background_executor()
                        .timer(Duration::from_millis(10))
                        .await;
                }
            })
            .detach();
        } else {
            panic!("broadcast thread already started");
        }
    }
}
