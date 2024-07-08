use std::ops::{Add, Div, Mul, Sub};

use chrono::offset;
use symphonia::core::sample;
use ux::{i24, u24};

use crate::{
    media::playback::{PlaybackFrame, Samples},
    util::{
        num_info::{BitCount, Bounds},
        ux_workaround::{PanicingFrom, WorkaroundInto},
    },
};

use super::format::SampleFormat;

fn integer_scale<T, U>(target: Vec<Vec<T>>) -> Vec<Vec<U>>
where
    T: Ord + Sized + Add + Sub + WorkaroundInto<i64> + Copy + BitCount + Bounds,
    U: Ord
        + Sized
        + Add
        + Sub
        + PanicingFrom<i64>
        + WorkaroundInto<i64>
        + Copy
        + std::fmt::Debug
        + BitCount
        + Bounds,
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

pub fn match_bit_depth(target_frame: PlaybackFrame, target_depth: SampleFormat) -> PlaybackFrame {
    let samples = match target_depth {
        SampleFormat::Float64 => todo!(),
        SampleFormat::Float32 => todo!(),
        SampleFormat::Signed32 => {
            let mut samples = vec![];

            match target_frame.samples {
                Samples::Float64(_) => todo!(),
                Samples::Float32(_) => todo!(),
                Samples::Signed32(v) => samples = v,
                Samples::Unsigned32(v) => samples = integer_scale(v),
                Samples::Signed24(v) => samples = integer_scale(v),
                Samples::Unsigned24(v) => samples = integer_scale(v),
                Samples::Signed16(v) => samples = integer_scale(v),
                Samples::Unsigned16(v) => samples = integer_scale(v),
                Samples::Signed8(v) => samples = integer_scale(v),
                Samples::Unsigned8(v) => samples = integer_scale(v),
                Samples::DSD(_) => unimplemented!(),
            }

            Samples::Signed32(samples)
        }
        SampleFormat::Unsigned32 => {
            let samples;

            match target_frame.samples {
                Samples::Float64(_) => todo!(),
                Samples::Float32(_) => todo!(),
                Samples::Signed32(v) => samples = integer_scale(v),
                Samples::Unsigned32(v) => samples = v,
                Samples::Signed24(v) => samples = integer_scale(v),
                Samples::Unsigned24(v) => samples = integer_scale(v),
                Samples::Signed16(v) => samples = integer_scale(v),
                Samples::Unsigned16(v) => samples = integer_scale(v),
                Samples::Signed8(v) => samples = integer_scale(v),
                Samples::Unsigned8(v) => samples = integer_scale(v),
                Samples::DSD(_) => unimplemented!(),
            }

            Samples::Unsigned32(samples)
        }
        SampleFormat::Signed24 => {
            let samples;

            match target_frame.samples {
                Samples::Float64(_) => todo!(),
                Samples::Float32(_) => todo!(),
                Samples::Signed32(v) => samples = integer_scale(v),
                Samples::Unsigned32(v) => samples = integer_scale(v),
                Samples::Signed24(v) => samples = v,
                Samples::Unsigned24(v) => samples = integer_scale(v),
                Samples::Signed16(v) => samples = integer_scale(v),
                Samples::Unsigned16(v) => samples = integer_scale(v),
                Samples::Signed8(v) => samples = integer_scale(v),
                Samples::Unsigned8(v) => samples = integer_scale(v),
                Samples::DSD(_) => unimplemented!(),
            }

            Samples::Signed24(samples)
        }
        SampleFormat::Unsigned24 => {
            let samples;

            match target_frame.samples {
                Samples::Float64(_) => todo!(),
                Samples::Float32(_) => todo!(),
                Samples::Signed32(v) => samples = integer_scale(v),
                Samples::Unsigned32(v) => samples = integer_scale(v),
                Samples::Signed24(v) => samples = integer_scale(v),
                Samples::Unsigned24(v) => samples = v,
                Samples::Signed16(v) => samples = integer_scale(v),
                Samples::Unsigned16(v) => samples = integer_scale(v),
                Samples::Signed8(v) => samples = integer_scale(v),
                Samples::Unsigned8(v) => samples = integer_scale(v),
                Samples::DSD(_) => unimplemented!(),
            }

            Samples::Unsigned24(samples)
        }
        SampleFormat::Signed16 => {
            let samples;

            match target_frame.samples {
                Samples::Float64(_) => todo!(),
                Samples::Float32(_) => todo!(),
                Samples::Signed32(v) => samples = integer_scale(v),
                Samples::Unsigned32(v) => samples = integer_scale(v),
                Samples::Signed24(v) => samples = integer_scale(v),
                Samples::Unsigned24(v) => samples = integer_scale(v),
                Samples::Signed16(v) => samples = v,
                Samples::Unsigned16(v) => samples = integer_scale(v),
                Samples::Signed8(v) => samples = integer_scale(v),
                Samples::Unsigned8(v) => samples = integer_scale(v),
                Samples::DSD(_) => unimplemented!(),
            }

            Samples::Signed16(samples)
        }
        SampleFormat::Unsigned16 => {
            let samples;

            match target_frame.samples {
                Samples::Float64(_) => todo!(),
                Samples::Float32(_) => todo!(),
                Samples::Signed32(v) => samples = integer_scale(v),
                Samples::Unsigned32(v) => samples = integer_scale(v),
                Samples::Signed24(v) => samples = integer_scale(v),
                Samples::Unsigned24(v) => samples = integer_scale(v),
                Samples::Signed16(v) => samples = integer_scale(v),
                Samples::Unsigned16(v) => samples = v,
                Samples::Signed8(v) => samples = integer_scale(v),
                Samples::Unsigned8(v) => samples = integer_scale(v),
                Samples::DSD(_) => unimplemented!(),
            }

            Samples::Unsigned16(samples)
        }
        SampleFormat::Signed8 => {
            let samples;

            match target_frame.samples {
                Samples::Float64(_) => todo!(),
                Samples::Float32(_) => todo!(),
                Samples::Signed32(v) => samples = integer_scale(v),
                Samples::Unsigned32(v) => samples = integer_scale(v),
                Samples::Signed24(v) => samples = integer_scale(v),
                Samples::Unsigned24(v) => samples = integer_scale(v),
                Samples::Signed16(v) => samples = integer_scale(v),
                Samples::Unsigned16(v) => samples = integer_scale(v),
                Samples::Signed8(v) => samples = v,
                Samples::Unsigned8(v) => samples = integer_scale(v),
                Samples::DSD(_) => unimplemented!(),
            }

            Samples::Signed8(samples)
        }
        SampleFormat::Unsigned8 => {
            let samples;

            match target_frame.samples {
                Samples::Float64(_) => todo!(),
                Samples::Float32(_) => todo!(),
                Samples::Signed32(v) => samples = integer_scale(v),
                Samples::Unsigned32(v) => samples = integer_scale(v),
                Samples::Signed24(v) => samples = integer_scale(v),
                Samples::Unsigned24(v) => samples = integer_scale(v),
                Samples::Signed16(v) => samples = integer_scale(v),
                Samples::Unsigned16(v) => samples = integer_scale(v),
                Samples::Signed8(v) => samples = integer_scale(v),
                Samples::Unsigned8(v) => samples = v,
                Samples::DSD(_) => unimplemented!(),
            }

            Samples::Unsigned8(samples)
        }
        SampleFormat::DSD => unimplemented!(),
        SampleFormat::Unsupported => panic!("target depth is unsupported"),
    };

    PlaybackFrame {
        samples,
        rate: target_frame.rate,
    }
}
