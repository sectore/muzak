use std::sync::mpsc::{Receiver, Sender};

use super::events::PlaybackCommand;

pub struct PlaybackInterface {
    pub commands_tx: Sender<PlaybackCommand>,
    pub events_rx: Receiver<PlaybackCommand>,
}
