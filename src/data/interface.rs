use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use gpui::{AppContext, Model};
use image::RgbaImage;

use crate::ui::models::Models;

use super::events::{DataCommand, DataEvent, ImageLayout, ImageType};

pub trait DataInterface {
    fn new(commands_tx: Sender<DataCommand>, events_rx: Receiver<DataEvent>) -> Self;
}

pub struct GPUIDataInterface {
    commands_tx: Sender<DataCommand>,
    events_rx: Option<Receiver<DataEvent>>,
}

impl gpui::Global for GPUIDataInterface {}

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
                        .timer(Duration::from_millis(50))
                        .await;
                }
            })
            .detach();
        } else {
            panic!("broadcast thread already started");
        }
    }
}
