use ux::{i24, u24};

use crate::devices::format::SampleFormat;

pub enum Samples {
    Float64(Vec<Vec<f64>>),
    Float32(Vec<Vec<f32>>),
    Signed32(Vec<Vec<i32>>),
    Unsigned32(Vec<Vec<u32>>),
    Signed24(Vec<Vec<i24>>),
    Unsigned24(Vec<Vec<u24>>),
    Signed16(Vec<Vec<i16>>),
    Unsigned16(Vec<Vec<u16>>),
    Signed8(Vec<Vec<i8>>),
    Unsigned8(Vec<Vec<u8>>),
    DSD(Vec<Vec<bool>>),
}

impl Samples {
    pub fn is_format(&self, format: SampleFormat) -> bool {
        match self {
            Samples::Float64(_) => format == SampleFormat::Float64,
            Samples::Float32(_) => format == SampleFormat::Float32,
            Samples::Signed32(_) => format == SampleFormat::Signed32,
            Samples::Unsigned32(_) => format == SampleFormat::Unsigned32,
            Samples::Signed24(_) => format == SampleFormat::Signed24,
            Samples::Unsigned24(_) => format == SampleFormat::Unsigned24,
            Samples::Signed16(_) => format == SampleFormat::Signed16,
            Samples::Unsigned16(_) => format == SampleFormat::Unsigned16,
            Samples::Signed8(_) => format == SampleFormat::Signed8,
            Samples::Unsigned8(_) => format == SampleFormat::Unsigned8,
            Samples::DSD(_) => format == SampleFormat::DSD,
        }
    }
}

pub enum SampleFromError {
    WrongFormat,
}

impl TryFrom<Samples> for Vec<Vec<f64>> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Float64(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<Vec<f32>> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Float32(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<Vec<u8>> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Unsigned8(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<Vec<u16>> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Unsigned16(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<Vec<u24>> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Unsigned24(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<Vec<u32>> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Unsigned32(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<Vec<i8>> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Signed8(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<Vec<i16>> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Signed16(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<Vec<i24>> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Signed24(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<Vec<i32>> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Signed32(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<Vec<bool>> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

pub struct PlaybackFrame {
    pub samples: Samples,
    pub rate: u32, // god forbid someone invents a PCM format that samples faster than 4 billion Hz
}
