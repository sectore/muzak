use std::{
    sync::mpsc::{Receiver, Sender},
    thread::sleep,
};

use crate::{
    devices::{
        builtin::cpal::CpalProvider,
        format::{ChannelSpec, FormatInfo},
        resample::Resampler,
        traits::{Device, DeviceProvider, OutputStream},
    },
    media::{
        builtin::symphonia::SymphoniaProvider, errors::PlaybackReadError, traits::MediaProvider,
    },
};

use super::{
    events::{PlaybackCommand, PlaybackEvent},
    interface::PlaybackInterface,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

pub struct PlaybackThread {
    commands_rx: Receiver<PlaybackCommand>,
    events_tx: Sender<PlaybackEvent>,
    media_provider: Option<Box<dyn MediaProvider>>,
    device_provider: Option<Box<dyn DeviceProvider>>,
    device: Option<Box<dyn Device>>,
    stream: Option<Box<dyn OutputStream>>,
    state: PlaybackState,
    resampler: Option<Resampler>,
    format: Option<FormatInfo>,
    queue: Vec<String>,
    queue_next: usize,
}

impl PlaybackThread {
    /// Starts the playback thread and returns the created interface.
    pub fn start<T: PlaybackInterface>() -> T {
        let (commands_tx, commands_rx) = std::sync::mpsc::channel();
        let (events_tx, events_rx) = std::sync::mpsc::channel();

        std::thread::Builder::new()
            .name("playback".to_string())
            .spawn(move || {
                let mut thread = PlaybackThread {
                    commands_rx,
                    events_tx,
                    media_provider: None,
                    device_provider: None,
                    device: None,
                    stream: None,
                    state: PlaybackState::Stopped,
                    resampler: None,
                    format: None,
                    queue: Vec::new(),
                    queue_next: 0,
                };

                thread.run();
            })
            .expect("could not start playback thread");

        T::new(commands_tx, events_rx)
    }

    pub fn run(&mut self) {
        // for now just throw in the default Providers and pick the default Device
        // TODO: Add a way to select the Device and MediaProvider
        self.device_provider = Some(Box::new(CpalProvider::default()));
        self.media_provider = Some(Box::new(SymphoniaProvider::default()));
        self.device = Some(
            self.device_provider
                .as_mut()
                .unwrap()
                .get_default_device()
                .unwrap(),
        );

        loop {
            self.main_loop();
        }
    }

    pub fn main_loop(&mut self) {
        self.command_intake();

        if self.state == PlaybackState::Playing {
            self.play_audio();
        } else {
            sleep(std::time::Duration::from_millis(10));
        }

        self.broadcast_events();
    }

    pub fn broadcast_events(&mut self) {
        if let Some(provider) = &mut self.media_provider {
            if provider.metadata_updated() {
                println!("Metadata updated");
                // TODO: proper error handling
                let metadata = provider.read_metadata().expect("failed to get metadata");
                self.events_tx
                    .send(PlaybackEvent::MetadataUpdate(Box::new(metadata.clone())))
                    .expect("unable to send event");

                let image = provider.read_image().expect("failed to decode image");
                self.events_tx
                    .send(PlaybackEvent::AlbumArtUpdate(image))
                    .expect("unable to send event");
            }
        }
    }

    pub fn command_intake(&mut self) {
        if let Ok(command) = self.commands_rx.try_recv() {
            match command {
                PlaybackCommand::Play => self.play(),
                PlaybackCommand::Pause => self.pause(),
                PlaybackCommand::Open(v) => self.open(&v),
                PlaybackCommand::Queue(v) => self.queue(&v),
                PlaybackCommand::QueueList(v) => self.queue_list(v),
                PlaybackCommand::Next => todo!(),
                PlaybackCommand::Previous => todo!(),
                PlaybackCommand::ClearQueue => todo!(),
                PlaybackCommand::Jump(_) => todo!(),
                PlaybackCommand::Seek(_) => todo!(),
                PlaybackCommand::SetVolume(_) => todo!(),
            }
        }
    }

    pub fn pause(&mut self) {
        if self.state == PlaybackState::Paused {
            return;
        }

        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;

            self.events_tx
                .send(PlaybackEvent::StateChanged(PlaybackState::Paused))
                .expect("unable to send event");
        }
    }

    pub fn play(&mut self) {
        if self.state == PlaybackState::Playing {
            return;
        }

        if self.state == PlaybackState::Paused {
            self.state = PlaybackState::Playing;
        }

        if self.state == PlaybackState::Stopped && !self.queue.is_empty() {
            self.open(&(self.queue[0].clone()));
            self.queue_next = 1;
        }

        // nothing to play, womp womp
    }

    pub fn open(&mut self, path: &String) {
        if self.stream.is_none() {
            // TODO: proper error handling
            // TODO: allow the user to pick a format on supported platforms
            let format = self.device.as_ref().unwrap().get_default_format().unwrap();
            self.stream = Some(self.device.as_mut().unwrap().open_device(format).unwrap());
        }

        if let Some(provider) = &mut self.media_provider {
            // TODO: proper error handling
            self.resampler = None;
            let src = std::fs::File::open(path).expect("failed to open media");
            provider.open(src, None).expect("unable to open file");
            provider.start_playback().expect("unable to start playback");
        }

        self.state = PlaybackState::Playing;
        self.events_tx
            .send(PlaybackEvent::SongChanged(path.clone(), 0 as f64))
            .expect("unable to send event");
        self.events_tx
            .send(PlaybackEvent::StateChanged(PlaybackState::Playing))
            .expect("unable to send event");
    }

    pub fn next(&mut self) {
        if self.queue_next < self.queue.len() {
            println!(
                "Next song: {} at {}",
                self.queue[self.queue_next], self.queue_next
            );
            let next_path = self.queue[self.queue_next].clone();
            self.open(&next_path);
            self.queue_next += 1;
        } else {
            if let Some(provider) = &mut self.media_provider {
                provider.stop_playback().expect("unable to stop playback");
                provider.close().expect("unable to close media");
            }
            self.state = PlaybackState::Stopped;
            self.events_tx
                .send(PlaybackEvent::StateChanged(PlaybackState::Stopped))
                .expect("unable to send event");
        }
    }

    pub fn queue(&mut self, path: &String) {
        let pre_len = self.queue.len();
        self.queue.push(path.clone());

        if self.state == PlaybackState::Stopped {
            self.open(path);
            self.queue_next = pre_len + 1;
            self.events_tx
                .send(PlaybackEvent::QueuePositionChanged(pre_len))
                .expect("unable to send event");
        }

        self.events_tx
            .send(PlaybackEvent::QueueUpdated(self.queue.clone()))
            .expect("unable to send event");
    }

    pub fn queue_list(&mut self, mut paths: Vec<String>) {
        let pre_len = self.queue.len();
        let first = paths.first().cloned();

        self.queue.append(&mut paths);

        if self.state == PlaybackState::Stopped {
            if let Some(first) = first {
                self.open(&first);
                self.queue_next = pre_len + 1;
            }
        }
    }

    pub fn play_audio(&mut self) {
        if let Some(stream) = &mut self.stream {
            if let Some(provider) = &mut self.media_provider {
                if self.resampler.is_none() {
                    // TODO: proper error handling
                    let first_samples = match provider.read_samples() {
                        Ok(samples) => samples,
                        Err(e) => match e {
                            PlaybackReadError::NothingOpen => {
                                panic!("thread state is invalid: no file open")
                            }
                            PlaybackReadError::NeverStarted => {
                                panic!("thread state is invalid: playback never started")
                            }
                            PlaybackReadError::EOF => {
                                self.next();
                                return;
                            }
                            PlaybackReadError::Unknown => return,
                            PlaybackReadError::DecodeFatal => panic!("fatal decoding error"),
                        },
                    };
                    let duration = provider.duration_frames().expect("can't get duration");
                    let device_format = stream.get_current_format().unwrap();

                    self.resampler = Some(Resampler::new(
                        first_samples.rate,
                        device_format.sample_rate,
                        duration,
                        // TODO: support getting channels from the bitmask
                        match device_format.channels {
                            ChannelSpec::Count(v) => v,
                            _ => 2,
                        },
                    ));
                    self.format = Some(device_format.clone());

                    let converted = self
                        .resampler
                        .as_mut()
                        .unwrap()
                        .convert_formats(first_samples, self.format.as_ref().unwrap());

                    stream
                        .submit_frame(converted)
                        .expect("failed to submit frames to stream");
                } else {
                    let samples = match provider.read_samples() {
                        Ok(samples) => samples,
                        Err(e) => match e {
                            PlaybackReadError::NothingOpen => {
                                panic!("thread state is invalid: no file open")
                            }
                            PlaybackReadError::NeverStarted => {
                                panic!("thread state is invalid: playback never started")
                            }
                            PlaybackReadError::EOF => {
                                self.next();
                                return;
                            }
                            PlaybackReadError::Unknown => return,
                            PlaybackReadError::DecodeFatal => panic!("fatal decoding error"),
                        },
                    };
                    let converted = self
                        .resampler
                        .as_mut()
                        .unwrap()
                        .convert_formats(samples, self.format.as_ref().unwrap());

                    stream
                        .submit_frame(converted)
                        .expect("failed to submit frames to stream");
                }
            }
        }
    }
}
