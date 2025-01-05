pub trait UnsignedInt:
    Clone
    + Copy
    + std::cmp::Eq
    + std::cmp::PartialEq
    + std::cmp::PartialOrd
    + std::cmp::Ord
    + std::ops::Add<Self, Output = Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Div<Self, Output = Self>
    + std::fmt::Debug
{
    const ZERO: Self;
    const MAX: Self;
    fn from_usize(value: usize) -> Self;
    fn to_usize(&self) -> usize;
}

macro_rules! impl_unsigned_int {
    ($T:ty) => {
        impl UnsignedInt for $T {
            fn from_usize(value: usize) -> Self {
                value as Self
            }
            fn to_usize(&self) -> usize {
                *self as usize
            }
            const ZERO: $T = 0;
            const MAX: $T = <$T>::MAX;
        }
    };
}

impl_unsigned_int!(usize);
impl_unsigned_int!(u64);
impl_unsigned_int!(u32);
impl_unsigned_int!(u16);
impl_unsigned_int!(u8);
