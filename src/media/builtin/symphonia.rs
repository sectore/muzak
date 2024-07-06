use std::fs::File;

use symphonia::core::{
    codecs::{DecoderOptions, CODEC_TYPE_NULL},
    errors::Error,
    formats::{FormatOptions, FormatReader},
    io::MediaSourceStream,
    meta::{MetadataOptions, StandardTagKey, Tag, Value},
    probe::{Hint, ProbeResult, ProbedMetadata},
};

use crate::media::{
    errors::{
        CloseError, MetadataError, OpenError, PlaybackReadError, PlaybackStartError,
        PlaybackStopError,
    },
    metadata::Metadata,
    playback::PlaybackFrame,
    traits::{MediaProvider, MetadataProvider, PlaybackProvider},
};

#[derive(Default)]
pub struct SymphoniaProvider {
    format: Option<Box<dyn FormatReader>>,
    current_metadata: Metadata,
}

const SYMPHONIA_SUPPORTED_FILETYPES: [&'static str; 7] = [
    "audio/ogg",
    "audio/aac",
    "audio/x-flac",
    "audio/x-wav",
    "audio/mpeg",
    "audio/m4a",
    "audio/x-aiff",
];

impl SymphoniaProvider {
    fn break_metadata(&mut self, tags: &[Tag]) {
        for tag in tags {
            match tag.std_key {
                Some(StandardTagKey::TrackTitle) => {
                    self.current_metadata.name = Some(tag.value.to_string())
                }
                Some(StandardTagKey::Artist) => {
                    self.current_metadata.artist = Some(tag.value.to_string())
                }
                Some(StandardTagKey::AlbumArtist) => {
                    self.current_metadata.album_artist = Some(tag.value.to_string())
                }
                Some(StandardTagKey::OriginalArtist) => {
                    self.current_metadata.original_artist = Some(tag.value.to_string())
                }
                Some(StandardTagKey::Composer) => {
                    self.current_metadata.composer = Some(tag.value.to_string())
                }
                Some(StandardTagKey::Album) => {
                    self.current_metadata.album = Some(tag.value.to_string())
                }
                Some(StandardTagKey::Genre) => {
                    self.current_metadata.genre = Some(tag.value.to_string())
                }
                Some(StandardTagKey::ContentGroup) => {
                    self.current_metadata.grouping = Some(tag.value.to_string())
                }
                Some(StandardTagKey::Bpm) => {
                    self.current_metadata.bpm = match &tag.value {
                        Value::String(v) => v.clone().parse().ok(),
                        Value::UnsignedInt(v) => Some(*v),
                        _ => None,
                    }
                }
                Some(StandardTagKey::Compilation) => {
                    self.current_metadata.compilation = match tag.value {
                        Value::Boolean(v) => v,
                        Value::Flag => true,
                        _ => false,
                    }
                }
                Some(StandardTagKey::Date) => {
                    self.current_metadata.date =
                        Some(dateparser::parse(&tag.value.to_string()).ok()).flatten();
                }
                Some(StandardTagKey::TrackNumber) => {
                    self.current_metadata.track_current = match &tag.value {
                        Value::String(v) => v.clone().parse().ok(),
                        Value::UnsignedInt(v) => Some(*v),
                        _ => None,
                    }
                }
                Some(StandardTagKey::TrackTotal) => {
                    self.current_metadata.track_max = match &tag.value {
                        Value::String(v) => v.clone().parse().ok(),
                        Value::UnsignedInt(v) => Some(*v),
                        _ => None,
                    }
                }
                Some(StandardTagKey::DiscNumber) => {
                    self.current_metadata.disc_current = match &tag.value {
                        Value::String(v) => v.clone().parse().ok(),
                        Value::UnsignedInt(v) => Some(*v),
                        _ => None,
                    }
                }
                Some(StandardTagKey::DiscTotal) => {
                    self.current_metadata.disc_max = match &tag.value {
                        Value::String(v) => v.clone().parse().ok(),
                        Value::UnsignedInt(v) => Some(*v),
                        _ => None,
                    }
                }
                _ => (),
            }
        }
    }

    fn read_base_metadata(&mut self, probed: &mut ProbeResult) {
        self.current_metadata = Metadata::default();

        if let Some(metadata) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
            self.break_metadata(metadata.tags());
        }

        if let Some(metadata) = probed.format.metadata().current() {
            self.break_metadata(metadata.tags());
        }
    }
}

impl MediaProvider for SymphoniaProvider {
    fn get_playback_provider(&mut self) -> Option<&mut impl PlaybackProvider> {
        Some(self)
    }

    fn get_metadata_provider(&mut self) -> Option<&mut impl MetadataProvider> {
        Some(self)
    }

    fn open(&mut self, file: File, ext: Option<String>) -> Result<(), OpenError> {
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let mut probed = if let Some(ext) = ext {
            let mut hint = Hint::new();
            hint.with_extension(&ext);

            symphonia::default::get_probe()
                .format(&hint, mss, &fmt_opts, &meta_opts)
                .map_err(|_| OpenError::UnsupportedFormat)?
        } else {
            let hint = Hint::new();

            symphonia::default::get_probe()
                .format(&hint, mss, &fmt_opts, &meta_opts)
                .map_err(|_| OpenError::UnsupportedFormat)?
        };

        self.read_base_metadata(&mut probed);

        self.format = Some(probed.format);

        Ok(())
    }

    fn close(&mut self) -> Result<(), CloseError> {
        todo!()
    }

    fn get_name(&self) -> &'static str {
        "Symphonia"
    }

    fn get_version(&self) -> &'static str {
        "0"
    }

    fn get_supported_mimetypes(&self) -> &'static [&'static str] {
        &SYMPHONIA_SUPPORTED_FILETYPES
    }
}

impl PlaybackProvider for SymphoniaProvider {
    fn start_playback(&mut self) -> Result<(), PlaybackStartError> {
        todo!()
    }

    fn stop_playback(&mut self) -> Result<(), PlaybackStopError> {
        todo!()
    }

    fn read_samples(&mut self) -> Result<PlaybackFrame, PlaybackReadError> {
        todo!()
    }
}

impl MetadataProvider for SymphoniaProvider {
    fn read_metadata(&mut self) -> Result<&Metadata, MetadataError> {
        if self.format.is_some() {
            // TODO: handle metadata updates
            Ok(&self.current_metadata)
        } else {
            Err(MetadataError::NothingOpen)
        }
    }
}
