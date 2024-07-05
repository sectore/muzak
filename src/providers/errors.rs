#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum OpenError {
    FileCorrupt,
    UnsupportedFormat,
    Unknown,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CloseError {
    Unknown,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlaybackStartError {
    NothingOpen,
    BrokenContainer,
    ContainerSupportedButNotCodec,
    Unknown,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlaybackStopError {
    NothingOpen,
    Unknown,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlaybackReadError {
    NothingOpen,
    NeverStarted,
    EOF,
    Unknown,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MetadataError {
    NothingOpen,
    Unknown,
}
