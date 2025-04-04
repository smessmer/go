use derive_more::{Debug, Display};
use derive_where::derive_where;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::utils::IntType;

pub trait BoardSize {
    const SIZE: usize;
    type Index: IntType;
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardSize3x3;
impl BoardSize for BoardSize3x3 {
    const SIZE: usize = 3;
    type Index = u8;
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardSize5x5;
impl BoardSize for BoardSize5x5 {
    const SIZE: usize = 5;
    type Index = u8;
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardSize7x7;
impl BoardSize for BoardSize7x7 {
    const SIZE: usize = 7;
    type Index = u8;
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardSize9x9;
impl BoardSize for BoardSize9x9 {
    const SIZE: usize = 9;
    type Index = u8; // Using u8 for 9x9 board, since 9*9=81 fits in u8
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardSize13x13;
impl BoardSize for BoardSize13x13 {
    const SIZE: usize = 13;
    type Index = u8; // Using u8 for 13x13 board, since 13*13=169 fits in u8
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardSize19x19;
impl BoardSize for BoardSize19x19 {
    const SIZE: usize = 19;
    type Index = u16; // Using u8 for 19x19 board, since 19*19=361 doesn't fit in u8
}

#[derive(Display)]
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumStones<BS: BoardSize> {
    #[display("{}")]
    num: BS::Index,
}

impl<BS: BoardSize> NumStones<BS> {
    pub const ZERO: Self = Self {
        num: <BS as BoardSize>::Index::ZERO,
    };

    pub const ONE: Self = Self {
        num: <BS as BoardSize>::Index::ONE,
    };

    pub fn from_usize(num: usize) -> Self {
        Self {
            num: <BS as BoardSize>::Index::try_from(num).unwrap(),
        }
    }

    pub fn into_usize(&self) -> usize {
        self.num.into()
    }
}

impl<BS: BoardSize> Add for NumStones<BS> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            num: self.num + other.num,
        }
    }
}

impl<BS: BoardSize> AddAssign for NumStones<BS> {
    fn add_assign(&mut self, other: Self) {
        self.num += other.num;
    }
}

impl<BS: BoardSize> Sub for NumStones<BS> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            num: self.num - other.num,
        }
    }
}

impl<BS: BoardSize> SubAssign for NumStones<BS> {
    fn sub_assign(&mut self, other: Self) {
        self.num -= other.num;
    }
}

#[derive(Display)]
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos<BS: BoardSize> {
    #[display("{}")]
    index: NumStones<BS>,
}

impl<BS: BoardSize> Pos<BS> {
    // TODO Rename to from_xy
    pub fn from_pointed_to(x: usize, y: usize) -> Self {
        Self::from_index(y * <BS as BoardSize>::SIZE + x)
    }

    pub fn from_index(index: usize) -> Self {
        Self {
            index: NumStones::<BS>::from_usize(index),
        }
    }

    pub fn index(&self) -> usize {
        self.index.into_usize()
    }
}
