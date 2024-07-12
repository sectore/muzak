use std::{
    sync::mpsc::{Receiver, Sender},
    thread::sleep,
    time::Duration,
};

use gpui::{AppContext, Model};
use image::RgbaImage;

use crate::{
    media::{metadata::Metadata, playback},
    ui::models::Models,
};

use super::{
    events::{PlaybackCommand, PlaybackEvent},
    thread::PlaybackState,
};

pub trait PlaybackInterface {
    fn new(commands_tx: Sender<PlaybackCommand>, events_rx: Receiver<PlaybackEvent>) -> Self;
}

pub struct GPUIPlaybackInterface {
    commands_tx: Sender<PlaybackCommand>,
    events_rx: Option<Receiver<PlaybackEvent>>,
    state: PlaybackState,
}

impl gpui::Global for GPUIPlaybackInterface {}

impl PlaybackInterface for GPUIPlaybackInterface {
    fn new(commands_tx: Sender<PlaybackCommand>, events_rx: Receiver<PlaybackEvent>) -> Self {
        Self {
            commands_tx,
            events_rx: Some(events_rx),
            state: PlaybackState::Stopped,
        }
    }
}

impl GPUIPlaybackInterface {
    pub fn play(&self) {
        self.commands_tx
            .send(PlaybackCommand::Play)
            .expect("could not send tx");
    }

    pub fn pause(&self) {
        self.commands_tx
            .send(PlaybackCommand::Pause)
            .expect("could not send tx");
    }

    pub fn open(&self, path: &str) {
        self.commands_tx
            .send(PlaybackCommand::Open(path.to_string()))
            .expect("could not send tx");
    }

    pub fn queue(&self, path: &str) {
        self.commands_tx
            .send(PlaybackCommand::Queue(path.to_string()))
            .expect("could not send tx");
    }

    pub fn queue_list(&self, paths: Vec<String>) {
        self.commands_tx
            .send(PlaybackCommand::QueueList(paths))
            .expect("could not send tx");
    }

    pub fn next(&self) {
        self.commands_tx
            .send(PlaybackCommand::Next)
            .expect("could not send tx");
    }

    pub fn previous(&self) {
        self.commands_tx
            .send(PlaybackCommand::Previous)
            .expect("could not send tx");
    }

    pub fn clear_queue(&self) {
        self.commands_tx
            .send(PlaybackCommand::ClearQueue)
            .expect("could not send tx");
    }

    pub fn jump(&self, index: usize) {
        self.commands_tx
            .send(PlaybackCommand::Jump(index))
            .expect("could not send tx");
    }

    pub fn seek(&self, position: f64) {
        self.commands_tx
            .send(PlaybackCommand::Seek(position))
            .expect("could not send tx");
    }

    pub fn set_volume(&self, volume: u8) {
        self.commands_tx
            .send(PlaybackCommand::SetVolume(volume))
            .expect("could not send tx");
    }

    pub fn get_state(&self) -> PlaybackState {
        self.state
    }

    pub fn start_broadcast_thread(&mut self, cx: &mut AppContext) {
        let mut events_rx = None;
        std::mem::swap(&mut self.events_rx, &mut events_rx);

        let metadata_model: Model<Metadata> = cx.global::<Models>().metadata.clone();
        let albumart_model: Model<Option<RgbaImage>> = cx.global::<Models>().albumart.clone();

        if let Some(events_rx) = events_rx {
            cx.spawn(|mut cx| async move {
                loop {
                    if let Ok(event) = events_rx.try_recv() {
                        match event {
                            PlaybackEvent::MetadataUpdate(v) => {
                                metadata_model
                                    .update(&mut cx, |m, cx| {
                                        *m = *v;
                                        cx.notify()
                                    })
                                    .expect("failed to update metadata");
                            }
                            PlaybackEvent::AlbumArtUpdate(v) => {
                                albumart_model
                                    .update(&mut cx, |m, cx| {
                                        *m = Some(v);
                                        cx.notify()
                                    })
                                    .expect("failed to update albumart");
                            }
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
