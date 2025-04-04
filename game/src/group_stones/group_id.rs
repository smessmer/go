use derive_more::Display;
use derive_where::derive_where;

use crate::{BoardSize, NumStones};

#[derive(Display)]
#[derive_where(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroupId<BS: BoardSize> {
    #[display("{}")]
    index: NumStones<BS>,
}

impl<BS: BoardSize> GroupId<BS> {
    pub const ZERO: Self = Self {
        index: NumStones::<BS>::ZERO,
    };

    pub fn from_usize(index: usize) -> Self {
        Self {
            index: NumStones::<BS>::from_usize(index),
        }
    }

    pub fn into_usize(&self) -> usize {
        self.index.into_usize()
    }

    pub fn increment(&mut self) {
        self.index += NumStones::<BS>::ONE;
    }
}
