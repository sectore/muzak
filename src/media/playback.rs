pub enum Samples {
    Float64(Vec<f64>),
    Float32(Vec<f32>),
    Signed32(Vec<i32>),
    Unsigned32(Vec<u32>),
    Signed24(Vec<i32>),
    Unsigned24(Vec<u32>),
    Signed16(Vec<i16>),
    Unsigned16(Vec<u16>),
    Signed8(Vec<i8>),
    Unsigned8(Vec<u8>),
    DSD(Vec<bool>),
}

pub struct PlaybackFrame {
    size: u32,
    samples: Samples,
    rate: u32, // god forbid someone invents a PCM format that samples faster than 4 billion Hz
    ending: bool,
}
