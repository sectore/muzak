use std::ops::{Add, Sub};

use ux::{i24, u24};

use crate::{
    media::playback::{PlaybackFrame, Samples},
    util::{
        num_info::{BitCount, Bounds},
        ux_workaround::{PanicingFrom, WorkaroundInto},
    },
};

use super::format::SampleFormat;

trait Samplable:
    Ord
    + Sized
    + Add
    + Sub
    + PanicingFrom<i64>
    + WorkaroundInto<i64>
    + Copy
    + std::fmt::Debug
    + BitCount
    + Bounds
{
}

impl Samplable for i32 {}
impl Samplable for i24 {}
impl Samplable for i16 {}
impl Samplable for i8 {}
impl Samplable for u32 {}
impl Samplable for u24 {}
impl Samplable for u16 {}
impl Samplable for u8 {}

fn integer_scale<T, U>(target: Vec<Vec<T>>) -> Vec<Vec<U>>
where
    T: Samplable,
    U: Samplable,
{
    let mut channels: Vec<Vec<U>> = vec![];

    for channel in target {
        let mut vec = vec![];

        // this doesn't work if someone tries to play like 7 bit audio but right now we don't
        // support that
        let offset = (2 as i64).pow((U::count() / 2) as u32) - 1;
        let factor = (T::unsigned_max() / U::unsigned_max()) as i64;

        for sample_original in channel {
            let sample: i64 = sample_original.into_workaround() + offset;
            let scaled_sample;

            if T::count() > U::count() {
                scaled_sample = sample / factor;
            } else {
                scaled_sample = sample * factor;
            }

            let clamped_sample = scaled_sample.clamp(
                U::min_val().into_workaround(),
                U::max_val().into_workaround(),
            );

            let result = U::panic_from(clamped_sample);

            vec.push(result);
        }

        channels.push(vec);
    }

    channels
}

pub fn convert_samples<T: Samplable>(target_frame: PlaybackFrame) -> Vec<Vec<T>> {
    match target_frame.samples {
        Samples::Float64(_) => todo!(),
        Samples::Float32(_) => todo!(),
        Samples::Signed32(v) => integer_scale(v),
        Samples::Unsigned32(v) => integer_scale(v),
        Samples::Signed24(v) => integer_scale(v),
        Samples::Unsigned24(v) => integer_scale(v),
        Samples::Signed16(v) => integer_scale(v),
        Samples::Unsigned16(v) => integer_scale(v),
        Samples::Signed8(v) => integer_scale(v),
        Samples::Unsigned8(v) => integer_scale(v),
        Samples::DSD(_) => unimplemented!(),
    }
}

pub fn match_bit_depth(target_frame: PlaybackFrame, target_depth: SampleFormat) -> PlaybackFrame {
    let rate = target_frame.rate;

    let samples = if !target_frame.samples.is_format(target_depth) {
        match target_depth {
            SampleFormat::Float64 => todo!(),
            SampleFormat::Float32 => todo!(),
            SampleFormat::Signed32 => Samples::Signed32(convert_samples(target_frame)),
            SampleFormat::Unsigned32 => Samples::Unsigned32(convert_samples(target_frame)),
            SampleFormat::Signed24 => Samples::Signed24(convert_samples(target_frame)),
            SampleFormat::Unsigned24 => Samples::Unsigned24(convert_samples(target_frame)),
            SampleFormat::Signed16 => Samples::Signed16(convert_samples(target_frame)),
            SampleFormat::Unsigned16 => Samples::Unsigned16(convert_samples(target_frame)),
            SampleFormat::Signed8 => Samples::Signed8(convert_samples(target_frame)),
            SampleFormat::Unsigned8 => Samples::Unsigned8(convert_samples(target_frame)),
            SampleFormat::DSD => unimplemented!(),
            SampleFormat::Unsupported => panic!("target depth is unsupported"),
        }
    } else {
        target_frame.samples
    };

    PlaybackFrame { samples, rate }
}
