use ux::{i24, u24};

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
