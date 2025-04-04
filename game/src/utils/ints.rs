use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub trait IntType:
    Copy
    + TryFrom<usize, Error: std::fmt::Debug>
    + Into<usize>
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + std::fmt::Debug
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Hash
    + Default
{
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! impl_int_type {
    ($t:ty) => {
        impl IntType for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
    };
}

impl_int_type!(u8);
impl_int_type!(u16);
impl_int_type!(usize);
