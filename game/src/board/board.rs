use bitvec::{BitArr, array::BitArray};
use std::ops::Index;

use super::{PlaceStoneError, Player};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board<const N: usize>
where
    [(); bitvec::mem::elts::<usize>(2 * N * N)]:,
{
    /// (x=0, y=0) origin is the top-left corner of the board
    /// cells[2 * (N*y+ )] is true if the cell at (x, y) is occupied.
    /// cells[2 * (N*y+x) + 1] can only be set if (x, y) is occupied and is true if the cell at (x, y) is black, false for white.
    cells: BitArr!(for 2*N*N),
}

impl<const N: usize> Board<N>
where
    [(); bitvec::mem::elts::<usize>(2 * N * N)]:,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            cells: BitArray::ZERO,
        }
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: Option<Player>) {
        let index = Self::index(x, y);
        self._set(index, value);
    }

    fn _set(&mut self, index: usize, value: Option<Player>) {
        self.cells.set(index, value.is_some());
        self.cells.set(
            index + 1,
            match value {
                None => false,
                Some(Player::White) => false,
                Some(Player::Black) => true,
            },
        );
    }

    #[inline]
    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        assert!(x < N && y < N, "Coordinates out of bounds");
        self._is_occupied(Self::index(x, y))
    }

    fn _is_occupied(&self, index: usize) -> bool {
        self.cells[index]
    }

    fn _is_black(&self, index: usize) -> bool {
        self.cells[index + 1]
    }

    #[inline]
    pub fn set_if_empty(&mut self, x: usize, y: usize, value: Player) -> Result<(), PlaceStoneError>
    where
        [(); bitvec::mem::elts::<usize>(2 * N * N)]:,
    {
        let index = Self::index(x, y);
        if self._is_occupied(index) {
            return Err(PlaceStoneError::CellOccupied);
        }

        self._set(index, Some(value));
        Ok(())
    }

    #[inline]
    const fn index(x: usize, y: usize) -> usize {
        assert!(x < N && y < N, "Coordinates out of bounds");
        2 * N * y + 2 * x
    }

    #[cfg(test)]
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, Option<Player>)>
    // TODO + ExactSizeIterator
    where
        [(); bitvec::mem::elts::<usize>(2 * N * N)]:,
    {
        (0..N).flat_map(move |y| (0..N).map(move |x| (x, y, self[(x, y)])))
    }
}

impl<const N: usize> Index<(usize, usize)> for Board<N>
where
    [(); bitvec::mem::elts::<usize>(2 * N * N)]:,
{
    type Output = Option<Player>;

    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let index = Self::index(x, y);
        if self._is_occupied(index) {
            if self._is_black(index) {
                &Some(Player::Black)
            } else {
                &Some(Player::White)
            }
        } else {
            &None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Player;

    use super::*;

    #[test]
    fn memory_size() {
        assert_eq!(96, std::mem::size_of::<Board<19>>());
        assert_eq!(48, std::mem::size_of::<Board<13>>());
        assert_eq!(24, std::mem::size_of::<Board<9>>());
    }

    #[test]
    fn empty_board() {
        let board = Board::<13>::new();
        for y in 0..13 {
            for x in 0..13 {
                assert_eq!(board[(x, y)], None, "Cell ({}, {}) should be empty", x, y);
                assert!(
                    !board.is_occupied(x, y),
                    "Cell ({}, {}) should not be occupied",
                    x,
                    y,
                );
            }
        }
    }

    #[test]
    fn set_and_get_cells() {
        let mut board = Board::<13>::new();

        board.set(0, 0, Some(Player::White));
        assert_eq!(board[(0, 0)], Some(Player::White));

        board.set(10, 10, Some(Player::Black));
        assert_eq!(board[(10, 10)], Some(Player::Black));

        board.set(12, 8, Some(Player::White));
        assert_eq!(board[(12, 8)], Some(Player::White));

        for y in 0..13 {
            for x in 0..13 {
                if (x, y) != (0, 0) && (x, y) != (10, 10) && (x, y) != (12, 8) {
                    assert_eq!(
                        board[(x, y)],
                        None,
                        "Cell ({}, {}) should still be empty",
                        x,
                        y
                    );
                }
            }
        }
    }
}
