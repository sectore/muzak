use ux::{i24, u24};

pub trait Bounds: Ord + Sized {
    fn max_val() -> Self;
    fn unsigned_max() -> u64;
    fn min_val() -> Self;
}

macro_rules! bounds {
    ($t:ty, $u:ty) => {
        impl Bounds for $t {
            fn max_val() -> Self {
                <$t>::MAX
            }

            fn unsigned_max() -> u64 {
                u64::from(<$u>::MAX)
            }

            fn min_val() -> Self {
                <$t>::MIN
            }
        }
    };
}

bounds!(u32, u32);
bounds!(u24, u24);
bounds!(u16, u16);
bounds!(u8, u8);
bounds!(i32, u32);
bounds!(i24, u24);
bounds!(i16, u16);
bounds!(i8, u8);

pub trait BitCount: Ord + Sized {
    fn count() -> u8;
}

macro_rules! count {
    ($t:ty, $c:expr) => {
        impl BitCount for $t {
            fn count() -> u8 {
                $c
            }
        }
    };
}

count!(u32, 32);
count!(u24, 24);
count!(u16, 16);
count!(u8, 8);
count!(i32, 32);
count!(i24, 24);
count!(i16, 16);
count!(i8, 8);
