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

#[derive_where(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumStones<BS: BoardSize> {
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

impl<BS: BoardSize> std::fmt::Display for NumStones<BS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.num)
    }
}

impl<BS: BoardSize> std::fmt::Debug for NumStones<BS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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

#[derive_where(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos<BS: BoardSize> {
    index: NumStones<BS>,
}

impl<BS: BoardSize> Pos<BS> {
    pub fn from_xy(x: usize, y: usize) -> Self {
        assert!(
            x < <BS as BoardSize>::SIZE && y < <BS as BoardSize>::SIZE,
            "Coordinates out of bounds"
        );
        Self::from_index(y * <BS as BoardSize>::SIZE + x)
    }

    pub fn from_index(index: usize) -> Self {
        assert!(
            index < <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE,
            "Index out of bounds"
        );
        Self::_from_index(index)
    }

    pub fn _from_index(index: usize) -> Self {
        Self {
            index: NumStones::<BS>::from_usize(index),
        }
    }

    pub fn index(&self) -> usize {
        self.index.into_usize()
    }

    pub fn x(&self) -> usize {
        self.index.into_usize() % <BS as BoardSize>::SIZE
    }

    pub fn y(&self) -> usize {
        self.index.into_usize() / <BS as BoardSize>::SIZE
    }

    pub fn left(&self) -> Option<Self> {
        if self.x() > 0 {
            Some(Self {
                index: self.index - NumStones::ONE,
            })
        } else {
            None
        }
    }

    pub fn right(&self) -> Option<Self> {
        if self.x() < <BS as BoardSize>::SIZE - 1 {
            Some(Self {
                index: self.index + NumStones::ONE,
            })
        } else {
            None
        }
    }

    pub fn up(&self) -> Option<Self> {
        if self.y() > 0 {
            Some(Self {
                index: self.index - NumStones::<BS>::from_usize(<BS as BoardSize>::SIZE),
            })
        } else {
            None
        }
    }

    pub fn down(&self) -> Option<Self> {
        if self.y() < <BS as BoardSize>::SIZE - 1 {
            Some(Self {
                index: self.index + NumStones::<BS>::from_usize(<BS as BoardSize>::SIZE),
            })
        } else {
            None
        }
    }

    pub fn all_positions() -> impl Iterator<Item = Self> + ExactSizeIterator {
        (0..<BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE).map(Self::_from_index)
    }
}

impl<BS: BoardSize> std::fmt::Display for Pos<BS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.x(), self.y())
    }
}

impl<BS: BoardSize> std::fmt::Debug for Pos<BS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
