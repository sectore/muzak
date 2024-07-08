use ux::{i24, u24};

pub trait WorkaroundInto<T> {
    fn into_workaround(self) -> T;
}

impl WorkaroundInto<i64> for u24 {
    fn into_workaround(self) -> i64 {
        u32::from(self) as i64
    }
}

macro_rules! workaround_into {
    ($t:ty) => {
        impl WorkaroundInto<i64> for $t {
            fn into_workaround(self) -> i64 {
                i64::from(self)
            }
        }
    };
}

workaround_into!(u32);
workaround_into!(u16);
workaround_into!(u8);
workaround_into!(i32);
workaround_into!(i24);
workaround_into!(i16);
workaround_into!(i8);

pub trait PanicingFrom<T> {
    fn panic_from(value: T) -> Self;
}

impl PanicingFrom<i64> for u24 {
    fn panic_from(value: i64) -> Self {
        u24::try_from(u64::try_from(value).expect("value out of u64 bounds"))
            .expect("value out of u24 bounds")
    }
}

macro_rules! panicing_from {
    ($t:ty) => {
        impl PanicingFrom<i64> for $t {
            fn panic_from(value: i64) -> Self {
                <$t>::try_from(value).expect("value out of bounds")
            }
        }
    };
}

panicing_from!(u32);
panicing_from!(u16);
panicing_from!(u8);
panicing_from!(i32);
panicing_from!(i24);
panicing_from!(i16);
panicing_from!(i8);
