use std::{path::PathBuf, sync::mpsc};

use ahash::AHashMap;
use sqlx::SqlitePool;

use crate::media::{
    builtin::symphonia::SymphoniaProvider,
    traits::{MediaPlugin, MediaProvider},
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum ScanEvent {
    DiscoverProgress(u64),
    DiscoverComplete(u64),
    ScanProgress(u64),
    ScanComplete,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ScanCommand {
    Scan,
    Stop,
}

pub struct ScanInterface {
    event_rx: mpsc::Receiver<ScanEvent>,
    command_tx: mpsc::Sender<ScanCommand>,
}

impl ScanInterface {
    pub(self) fn new(
        event_rx: mpsc::Receiver<ScanEvent>,
        command_tx: mpsc::Sender<ScanCommand>,
    ) -> Self {
        ScanInterface {
            event_rx,
            command_tx,
        }
    }

    pub fn scan(&self) {
        self.command_tx
            .send(ScanCommand::Scan)
            .expect("could not send tx");
    }

    pub fn stop(&self) {
        self.command_tx
            .send(ScanCommand::Stop)
            .expect("could not send tx");
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScanState {
    Idle,
    Discovering,
    Scanning,
}

pub struct ScanThread {
    event_tx: mpsc::Sender<ScanEvent>,
    command_rx: mpsc::Receiver<ScanCommand>,
    pool: SqlitePool,
    visited: Vec<PathBuf>,
    discovered: Vec<PathBuf>,
    to_process: Vec<PathBuf>,
    scan_state: ScanState,
    provider_table: Vec<(&'static [&'static str], Box<dyn MediaProvider>)>,
}

fn build_provider_table() -> Vec<(&'static [&'static str], Box<dyn MediaProvider>)> {
    // TODO: dynamic plugin loading
    vec![(
        SymphoniaProvider::SUPPORTED_EXTENSIONS,
        Box::new(SymphoniaProvider::default()),
    )]
}

impl ScanThread {
    pub fn start(pool: SqlitePool) -> ScanInterface {
        let (commands_tx, commands_rx) = std::sync::mpsc::channel();
        let (events_tx, events_rx) = std::sync::mpsc::channel();

        let thread = std::thread::Builder::new()
            .name("playback".to_string())
            .spawn(move || {
                let mut thread = ScanThread {
                    event_tx: events_tx,
                    command_rx: commands_rx,
                    pool,
                    visited: Vec::new(),
                    discovered: Vec::new(),
                    to_process: Vec::new(),
                    scan_state: ScanState::Idle,
                    provider_table: build_provider_table(),
                };

                thread.run();
            })
            .expect("could not start playback thread");

        ScanInterface::new(events_rx, commands_tx)
    }

    fn run(&mut self) {
        loop {
            self.read_commands();

            if self.scan_state == ScanState::Discovering {
                self.discover();
            } else if self.scan_state == ScanState::Scanning {
                self.scan();
            } else {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    fn read_commands(&mut self) {
        while let Ok(command) = self.command_rx.try_recv() {
            match command {
                ScanCommand::Scan => {
                    if self.scan_state == ScanState::Idle {
                        self.scan_state = ScanState::Discovering;
                    }
                }
                ScanCommand::Stop => {
                    self.scan_state = ScanState::Idle;
                    self.visited.clear();
                    self.discovered.clear();
                    self.to_process.clear();
                }
            }
        }
    }

    fn discover(&mut self) {}

    fn scan(&mut self) {}
}
