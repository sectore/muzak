use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread::sleep,
};

use crate::{
    devices::{
        builtin::cpal::CpalProvider,
        format::ChannelSpec,
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
    current_media_provider: Option<Box<dyn MediaProvider>>,
    current_device_provider: Option<Box<dyn DeviceProvider>>,
    current_device: Option<Box<dyn Device>>,
    current_stream: Option<Box<dyn OutputStream>>,
    current_state: PlaybackState,
    current_resampler: Option<Resampler>,
    queue: Vec<String>,
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
                current_media_provider: None,
                current_device_provider: None,
                current_device: None,
                current_stream: None,
                current_state: PlaybackState::Stopped,
                queue: Vec::new(),
                current_resampler: None,
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
        self.current_device_provider = Some(Box::new(CpalProvider::default()));
        self.current_media_provider = Some(Box::new(SymphoniaProvider::default()));
        self.current_device = Some(
            self.current_device_provider
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

        if PlaybackState::Playing == self.current_state {
            self.play_audio();
        } else {
            sleep(std::time::Duration::from_millis(100));
        }
    }

    pub fn command_intake(&mut self) {
        if let Ok(command) = self.commands_rx.try_recv() {
            match command {
                PlaybackCommand::Play => todo!(),
                PlaybackCommand::Pause => todo!(),
                PlaybackCommand::Open(_) => todo!(),
                PlaybackCommand::Queue(_) => todo!(),
                PlaybackCommand::QueueList(_) => todo!(),
                PlaybackCommand::Next => todo!(),
                PlaybackCommand::Previous => todo!(),
                PlaybackCommand::ClearQueue => todo!(),
                PlaybackCommand::Jump(_) => todo!(),
                PlaybackCommand::Seek(_) => todo!(),
                PlaybackCommand::SetVolume(_) => todo!(),
            }
        }
    }

    pub fn play_audio(&mut self) {
        if let Some(device) = &mut self.current_device {
            if let Some(stream) = &mut self.current_stream {
                if let Some(provider) = &mut self.current_media_provider {
                    if self.current_resampler.is_none() {
                        let first_samples =
                            provider.read_samples().expect("unable to read samples");
                        let duration = provider.duration_frames().expect("can't get duration");
                        let device_format = stream.get_current_format().unwrap();

                        self.current_resampler = Some(Resampler::new(
                            first_samples.rate,
                            device_format.sample_rate,
                            duration,
                            // TODO: support getting channels from the bitmask
                            match device_format.channels {
                                ChannelSpec::Count(v) => v,
                                _ => 2,
                            },
                        ));

                        let converted = self
                            .current_resampler
                            .as_mut()
                            .unwrap()
                            .convert_formats(first_samples, device_format.clone());

                        stream.submit_frame(converted);
                    }
                }
            }
        }
    }
}
