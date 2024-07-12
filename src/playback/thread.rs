use std::{
    collections::HashMap,
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
        builtin::symphonia::SymphoniaProvider,
        traits::{MediaPlugin, MediaProvider},
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
    media_provider_index: HashMap<&'static str, Vec<&'static str>>,
    device_provider_index: Vec<&'static str>,
    commands_rx: Receiver<PlaybackCommand>,
    events_tx: Sender<PlaybackCommand>,
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
    pub fn start() -> PlaybackInterface {
        let (commands_tx, commands_rx) = std::sync::mpsc::channel();
        let (events_tx, events_rx) = std::sync::mpsc::channel();
        let mut media_provider_index: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

        for i in SymphoniaProvider::SUPPORTED_MIMETYPES {
            media_provider_index.insert(i, vec![SymphoniaProvider::NAME]);
        }

        std::thread::spawn(move || {
            let mut thread = PlaybackThread {
                media_provider_index,
                device_provider_index: vec!["cpal"],
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
        });

        PlaybackInterface {
            commands_tx,
            events_rx,
        }
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

        if PlaybackState::Playing == self.state {
            self.play_audio();
        } else {
            sleep(std::time::Duration::from_millis(50));
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
            let src = std::fs::File::open(path).expect("failed to open media");
            provider.open(src, None).expect("unable to open file");
            provider.start_playback().expect("unable to start playback");
        }
    }

    pub fn queue(&mut self, path: &String) {
        self.queue.push(path.clone());

        if self.state == PlaybackState::Stopped {
            self.open(path);
        }
    }

    pub fn queue_list(&mut self, mut paths: Vec<String>) {
        let first = paths.first().cloned();

        self.queue.append(&mut paths);

        if self.state == PlaybackState::Stopped {
            if let Some(first) = first {
                self.open(&first);
            }
        }
    }

    pub fn play_audio(&mut self) {
        if let Some(stream) = &mut self.stream {
            if let Some(provider) = &mut self.media_provider {
                if self.resampler.is_none() {
                    // TODO: proper error handling
                    let first_samples = provider.read_samples().expect("unable to read samples");
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

                    stream.submit_frame(converted);
                } else {
                    let samples = provider.read_samples().expect("unable to read samples");
                    let converted = self
                        .resampler
                        .as_mut()
                        .unwrap()
                        .convert_formats(samples, self.format.as_ref().unwrap());

                    stream.submit_frame(converted);
                }
            }
        }
    }
}
