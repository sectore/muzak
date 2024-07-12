use std::{fs::File, io::Cursor};

use image::RgbaImage;
use symphonia::{
    core::{
        audio::{AudioBufferRef, Signal},
        codecs::{Decoder, DecoderOptions, CODEC_TYPE_NULL},
        errors::Error,
        formats::{FormatOptions, FormatReader},
        io::MediaSourceStream,
        meta::{MetadataOptions, StandardTagKey, Tag, Value, Visual},
        probe::{Hint, ProbeResult},
    },
    default::get_codecs,
};
use ux::{i24, u24};

use crate::media::{
    errors::{
        CloseError, DurationError, MetadataError, OpenError, PlaybackReadError, PlaybackStartError,
        PlaybackStopError,
    },
    metadata::Metadata,
    playback::{PlaybackFrame, Samples},
    traits::{MediaPlugin, MediaProvider},
};

#[derive(Default)]
pub struct SymphoniaProvider {
    format: Option<Box<dyn FormatReader>>,
    current_metadata: Metadata,
    current_track: u32,
    current_duration: u64,
    decoder: Option<Box<dyn Decoder>>,
    pending_metadata_update: bool,
    last_image: Option<Visual>,
}

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
        self.last_image = None;

        if let Some(metadata) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
            self.break_metadata(metadata.tags());
            if !metadata.visuals().is_empty() {
                self.last_image = Some(metadata.visuals()[0].clone());
            }
        }

        if let Some(metadata) = probed.format.metadata().current() {
            self.break_metadata(metadata.tags());
            if !metadata.visuals().is_empty() {
                self.last_image = Some(metadata.visuals()[0].clone());
            }
        }

        self.pending_metadata_update = true;
    }
}

impl MediaProvider for SymphoniaProvider {
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
        self.stop_playback().expect("invalid outcome");
        self.current_metadata = Metadata::default();
        self.format = None;
        Ok(())
    }

    fn start_playback(&mut self) -> Result<(), PlaybackStartError> {
        if let Some(format) = &self.format {
            let track = format
                .tracks()
                .iter()
                .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
                .ok_or(PlaybackStartError::NothingToPlay)?;

            self.current_track = track.id;

            let dec_opts: DecoderOptions = Default::default();
            self.decoder = Some(
                get_codecs()
                    .make(&track.codec_params, &dec_opts)
                    .map_err(|_| PlaybackStartError::Undecodable)?,
            );

            Ok(())
        } else {
            Err(PlaybackStartError::NothingOpen)
        }
    }

    fn stop_playback(&mut self) -> Result<(), PlaybackStopError> {
        self.current_track = 0;
        self.decoder = None;

        Ok(())
    }

    fn read_samples(&mut self) -> Result<PlaybackFrame, PlaybackReadError> {
        if let Some(format) = &mut self.format {
            // this has a loop because the next packet may not be from the current track
            loop {
                let packet = match format.next_packet() {
                    Ok(packet) => packet,
                    Err(Error::ResetRequired) => return Err(PlaybackReadError::EOF),
                    Err(_) => {
                        // TODO: Handle better
                        return Err(PlaybackReadError::EOF);
                    }
                };

                while !format.metadata().is_latest() {
                    // TODO: handle metadata updates
                    format.metadata().pop();
                }

                if packet.track_id() != self.current_track {
                    continue;
                }

                if let Some(decoder) = &mut self.decoder {
                    match decoder.decode(&packet) {
                        Ok(decoded) => {
                            let rate = decoded.spec().rate;
                            let channel_count = decoded.spec().channels.count();
                            self.current_duration = decoded.capacity() as u64;

                            match decoded {
                                AudioBufferRef::U8(v) => {
                                    let mut samples: Vec<Vec<u8>> = Vec::new();

                                    for i in 0..channel_count {
                                        samples.push(Vec::new());
                                        for sample in v.chan(i) {
                                            samples[i].push(*sample);
                                        }
                                    }

                                    return Ok(PlaybackFrame {
                                        rate,
                                        samples: Samples::Unsigned8(samples),
                                    });
                                }
                                AudioBufferRef::U16(v) => {
                                    let mut samples: Vec<Vec<u16>> = Vec::new();

                                    for i in 0..channel_count {
                                        samples.push(Vec::new());
                                        for sample in v.chan(i) {
                                            samples[i].push(*sample);
                                        }
                                    }

                                    return Ok(PlaybackFrame {
                                        rate,
                                        samples: Samples::Unsigned16(samples),
                                    });
                                }
                                AudioBufferRef::U24(v) => {
                                    let mut samples: Vec<Vec<u24>> = Vec::new();

                                    for i in 0..channel_count {
                                        samples.push(Vec::new());
                                        for sample in v.chan(i) {
                                            samples[i].push(
                                                u24::try_from(sample.0)
                                                    .expect("24bit number is not 24bits long"),
                                            );
                                        }
                                    }

                                    return Ok(PlaybackFrame {
                                        rate,
                                        samples: Samples::Unsigned24(samples),
                                    });
                                }
                                AudioBufferRef::U32(v) => {
                                    let mut samples: Vec<Vec<u32>> = Vec::new();

                                    for i in 0..channel_count {
                                        samples.push(Vec::new());
                                        for sample in v.chan(i) {
                                            samples[i].push(*sample);
                                        }
                                    }

                                    return Ok(PlaybackFrame {
                                        rate,
                                        samples: Samples::Unsigned32(samples),
                                    });
                                }
                                AudioBufferRef::S8(v) => {
                                    let mut samples: Vec<Vec<i8>> = Vec::new();

                                    for i in 0..channel_count {
                                        samples.push(Vec::new());
                                        for sample in v.chan(i) {
                                            samples[i].push(*sample);
                                        }
                                    }

                                    return Ok(PlaybackFrame {
                                        rate,
                                        samples: Samples::Signed8(samples),
                                    });
                                }
                                AudioBufferRef::S16(v) => {
                                    let mut samples: Vec<Vec<i16>> = Vec::new();

                                    for i in 0..channel_count {
                                        samples.push(Vec::new());
                                        for sample in v.chan(i) {
                                            samples[i].push(*sample);
                                        }
                                    }

                                    return Ok(PlaybackFrame {
                                        rate,
                                        samples: Samples::Signed16(samples),
                                    });
                                }
                                AudioBufferRef::S24(v) => {
                                    let mut samples: Vec<Vec<i24>> = Vec::new();

                                    for i in 0..channel_count {
                                        samples.push(Vec::new());
                                        for sample in v.chan(i) {
                                            samples[i].push(
                                                i24::try_from(sample.0)
                                                    .expect("24bit number is not 24bits long"),
                                            );
                                        }
                                    }

                                    return Ok(PlaybackFrame {
                                        rate,
                                        samples: Samples::Signed24(samples),
                                    });
                                }
                                AudioBufferRef::S32(v) => {
                                    let mut samples: Vec<Vec<i32>> = Vec::new();

                                    for i in 0..channel_count {
                                        samples.push(Vec::new());
                                        for sample in v.chan(i) {
                                            samples[i].push(*sample);
                                        }
                                    }

                                    return Ok(PlaybackFrame {
                                        rate,
                                        samples: Samples::Signed32(samples),
                                    });
                                }
                                AudioBufferRef::F32(v) => {
                                    let mut samples: Vec<Vec<f32>> = Vec::new();

                                    for i in 0..channel_count {
                                        samples.push(Vec::new());
                                        for sample in v.chan(i) {
                                            samples[i].push(*sample);
                                        }
                                    }

                                    return Ok(PlaybackFrame {
                                        rate,
                                        samples: Samples::Float32(samples),
                                    });
                                }
                                AudioBufferRef::F64(v) => {
                                    let mut samples: Vec<Vec<f64>> = Vec::new();

                                    for i in 0..channel_count {
                                        samples.push(Vec::new());
                                        for sample in v.chan(i) {
                                            samples[i].push(*sample);
                                        }
                                    }

                                    return Ok(PlaybackFrame {
                                        rate,
                                        samples: Samples::Float64(samples),
                                    });
                                }
                            }
                        }
                        Err(Error::IoError(_)) | Err(Error::DecodeError(_)) => {
                            continue;
                        }
                        Err(_) => {
                            return Err(PlaybackReadError::DecodeFatal);
                        }
                    }
                } else {
                    return Err(PlaybackReadError::NeverStarted);
                }
            }
        } else {
            Err(PlaybackReadError::NothingOpen)
        }
    }

    fn duration_frames(&self) -> Result<u64, DurationError> {
        if self.decoder.is_none() {
            Err(DurationError::NothingOpen)
        } else if self.current_duration == 0 {
            Err(DurationError::NeverDecoded)
        } else {
            Ok(self.current_duration)
        }
    }

    fn read_metadata(&mut self) -> Result<&Metadata, MetadataError> {
        self.pending_metadata_update = false;

        if self.format.is_some() {
            Ok(&self.current_metadata)
        } else {
            Err(MetadataError::NothingOpen)
        }
    }

    fn metadata_updated(&self) -> bool {
        self.pending_metadata_update
    }

    fn read_image(&mut self) -> Result<Option<RgbaImage>, MetadataError> {
        if self.format.is_some() {
            if let Some(visual) = &self.last_image {
                let image = image::io::Reader::new(Cursor::new(visual.data.clone()))
                    .with_guessed_format()
                    .map_err(|_| MetadataError::Unknown)?
                    .decode()
                    .map_err(|_| MetadataError::Unknown)?;
                Ok(Some(image.into_rgba8()))
            } else {
                Ok(None)
            }
        } else {
            Err(MetadataError::NothingOpen)
        }
    }
}

impl MediaPlugin for SymphoniaProvider {
    const NAME: &'static str = "Symphonia";

    const VERSION: &'static str = "0.1.0";

    const SUPPORTED_MIMETYPES: &'static [&'static str] = &[
        "audio/ogg",
        "audio/aac",
        "audio/x-flac",
        "audio/x-wav",
        "audio/mpeg",
        "audio/m4a",
        "audio/x-aiff",
    ];
}
