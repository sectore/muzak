use std::fs::File;

use super::{
    errors::{
        CloseError, DurationError, MetadataError, OpenError, PlaybackReadError, PlaybackStartError,
        PlaybackStopError,
    },
    metadata::Metadata,
    playback::PlaybackFrame,
};

pub trait MediaPlugin: MediaProvider {
    const NAME: &'static str;
    const VERSION: &'static str;
    const SUPPORTED_MIMETYPES: &'static [&'static str];
}

pub trait MediaProvider {
    /// Requests the Provider open a file.
    fn open(&mut self, file: File, ext: Option<String>) -> Result<(), OpenError>;

    /// Informs the Provider that the currently opened file is no longer needed.
    fn close(&mut self) -> Result<(), CloseError>;

    /// Requests the Provider prepare for samples to be read from the file and played to the user.
    fn start_playback(&mut self) -> Result<(), PlaybackStartError>;

    /// Informs the Provider that playback has ended and no more samples will be read.
    fn stop_playback(&mut self) -> Result<(), PlaybackStopError>;

    /// Requests the Provider provide samples for playback.
    fn read_samples(&mut self) -> Result<PlaybackFrame, PlaybackReadError>;

    /// Requests the provider to provide the duration of a PlaybackFrame in frames (samples). This
    /// value is no longer valid after stop_playback is called, or the file is closed.
    fn duration_frames(&self) -> Result<u64, DurationError>;

    /// Requests the Provider retrieve the metatdata for the currently opened file.
    fn read_metadata(&mut self) -> Result<&Metadata, MetadataError>;

    /// Retrieves whether or not there has been a metadata update since the last call to
    /// read_metadata.
    fn metadata_updated(&self) -> bool;
}
