use std::fs::File;

use super::{
    errors::{
        CloseError, MetadataError, OpenError, PlaybackReadError, PlaybackStartError,
        PlaybackStopError,
    },
    metadata::Metadata,
    playback::PlaybackFrame,
};

pub trait Provider {
    // Plugin Functionality

    /// Upgrades a Provider reference to a PlaybackProvider reference, if the Provider supports
    /// playback.
    fn get_playback_provider(&mut self) -> Option<&mut impl PlaybackProvider>;

    /// Upgrades a Provider reference to a MetadataProvider reference, if the Provider supports
    /// metadata retrival.
    fn get_metadata_provider(&mut self) -> Option<&mut impl MetadataProvider>;

    /// Requests the Provider open a file.
    fn open(&mut self, file: File, ext: Option<String>) -> Result<(), OpenError>;

    /// Informs the Provider that the currently opened file is no longer needed.
    fn close(&mut self) -> Result<(), CloseError>;

    // Plugin Metadata

    /// Returns the Provider's name.
    fn get_name(&self) -> &'static str;

    /// Returns the Provider's version.
    fn get_version(&self) -> &'static str;

    /// Returns the Provider's supported mimetypes, used for determining whether or not to use a
    /// Provider for playback of a particular file.
    fn get_supported_mimetypes(&self) -> &'static [&'static str];
}

pub trait PlaybackProvider {
    /// Requests the Provider prepare for samples to be read from the file and played to the user.
    fn start_playback(&mut self) -> Result<(), PlaybackStartError>;

    /// Informs the Provider that playback has ended and no more samples will be read.
    fn stop_playback(&mut self) -> Result<(), PlaybackStopError>;

    /// Requests the Provider provide samples for playback.
    fn read_samples(&mut self) -> Result<PlaybackFrame, PlaybackReadError>;
}

pub trait MetadataProvider {
    /// Requests the Provider retrieve the metatdata for the currently opened file.
    fn read_metadata(&mut self) -> Result<&Metadata, MetadataError>;
}
