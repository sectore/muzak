use std::{fs, path::PathBuf, sync::mpsc};

use ahash::AHashMap;
use sqlx::SqlitePool;
use tracing::debug;

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
    base_paths: Vec<PathBuf>,
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

fn retrieve_base_paths() -> Vec<PathBuf> {
    // TODO: user-defined base paths
    // TODO: we should also probably check if these directories exist
    let system_music = directories::UserDirs::new()
        .unwrap()
        .audio_dir()
        .unwrap()
        .to_path_buf();

    vec![system_music]
}

impl ScanThread {
    pub fn start(pool: SqlitePool) -> ScanInterface {
        let (commands_tx, commands_rx) = std::sync::mpsc::channel();
        let (events_tx, events_rx) = std::sync::mpsc::channel();

        let thread = std::thread::Builder::new()
            .name("scanner".to_string())
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
                    base_paths: retrieve_base_paths(),
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
                        self.discovered = self.base_paths.clone();
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

        if self.scan_state == ScanState::Discovering {
            self.discover();
        } else if self.scan_state == ScanState::Scanning {
            self.scan();
        } else {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    fn file_is_scannable(&self, path: &PathBuf) -> bool {
        for (extensions, _) in self.provider_table.iter() {
            for extension in extensions.iter() {
                if path.extension().unwrap() == *extension {
                    return true;
                }
            }
        }

        false
    }

    fn discover(&mut self) {
        if self.discovered.is_empty() {
            self.scan_state = ScanState::Scanning;
            return;
        }

        let path = self.discovered.pop().unwrap();

        if self.visited.contains(&path) {
            return;
        }

        let paths = fs::read_dir(&path).unwrap();

        for paths in paths {
            // TODO: handle errors
            // this might be slower than just reading the path directly but this prevents loops
            let path = paths.unwrap().path().canonicalize().unwrap();
            if path.is_dir() {
                self.discovered.push(path);
            } else if self.file_is_scannable(&path) {
                self.to_process.push(path);
            }
        }

        self.visited.push(path.clone());
    }

    fn scan(&mut self) {
        // TODO: actually scan the files
        debug!("{:?}", self.to_process);
        self.scan_state = ScanState::Idle;
    }
}
