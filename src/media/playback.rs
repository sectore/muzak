pub enum Samples {
    Float64(Vec<f64>),
    Float32(Vec<f32>),
    Signed32(Vec<i32>),
    Unsigned32(Vec<u32>),
    Signed16(Vec<i16>),
    Unsigned16(Vec<u16>),
    Signed8(Vec<i8>),
    Unsigned8(Vec<u8>),
    DSD(Vec<bool>),
}

pub enum SampleFromError {
    WrongFormat,
}

impl TryFrom<Samples> for Vec<f64> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Float64(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<f32> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Float32(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<u8> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Unsigned8(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<u16> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Unsigned16(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<u32> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Unsigned32(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<i8> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Signed8(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<i16> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Signed16(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<i32> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            Samples::Signed32(v) => Ok(v),
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

impl TryFrom<Samples> for Vec<bool> {
    type Error = SampleFromError;

    fn try_from(value: Samples) -> Result<Self, Self::Error> {
        match value {
            _ => Err(SampleFromError::WrongFormat),
        }
    }
}

pub struct PlaybackFrame {
    size: u32,
    samples: Samples,
    rate: u32, // god forbid someone invents a PCM format that samples faster than 4 billion Hz
    ending: bool,
}
