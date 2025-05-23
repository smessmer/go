use bitvec::{array::BitArray, order::Lsb0};
use std::fmt::Debug;
use std::ops::Index;

use super::{PlaceStoneError, Player, Pos, pos::BoardSize};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Board<BS: BoardSize>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
{
    /// (x=0, y=0) origin is the top-left corner of the board
    /// cells[2 * (BOARD_SIZE*y+ )] is true if the cell at (x, y) is occupied.
    /// cells[2 * (BOARD_SIZE*y+x) + 1] can only be set if (x, y) is occupied and is true if the cell at (x, y) is black, false for white.
    cells: BitArray<
        [usize; bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)],
        Lsb0,
    >,
}

impl<BS: BoardSize> Debug for Board<BS>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Board(")?;
        for y in 0..<BS as BoardSize>::SIZE {
            for x in 0..<BS as BoardSize>::SIZE {
                let cell = self[Pos::from_xy(x, y)];
                match cell {
                    Some(Player::Black) => write!(f, "● ")?,
                    Some(Player::White) => write!(f, "○ ")?,
                    None => write!(f, "_ ")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, ")")?;
        Ok(())
    }
}

impl<BS: BoardSize> Board<BS>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            cells: BitArray::ZERO,
        }
    }

    #[inline]
    pub fn set(&mut self, pos: Pos<BS>, value: Option<Player>) {
        let index = Self::index(pos);
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
    pub fn is_occupied(&self, pos: Pos<BS>) -> bool {
        self._is_occupied(Self::index(pos))
    }

    fn _is_occupied(&self, index: usize) -> bool {
        self.cells[index]
    }

    fn _is_black(&self, index: usize) -> bool {
        self.cells[index + 1]
    }

    #[inline]
    pub fn set_if_empty(&mut self, pos: Pos<BS>, value: Player) -> Result<(), PlaceStoneError> {
        let index = Self::index(pos);
        if self._is_occupied(index) {
            return Err(PlaceStoneError::CellOccupied);
        }

        self._set(index, Some(value));
        Ok(())
    }

    #[inline]
    fn index(pos: Pos<BS>) -> usize {
        let pos_index = pos.index();
        2 * pos_index
    }

    #[cfg(test)]
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (Pos<BS>, Option<Player>)>
// TODO + ExactSizeIterator
    {
        (0..<BS as BoardSize>::SIZE).flat_map(move |y| {
            (0..<BS as BoardSize>::SIZE).map(move |x| {
                let pos = Pos::from_xy(x, y);
                (pos, self[pos])
            })
        })
    }

    pub fn from_str(input: &str) -> Result<Self, String> {
        let mut board = Board::<BS>::new();
        let mut input = input.chars().peekable();
        for y in 0..<BS as BoardSize>::SIZE {
            for x in 0..<BS as BoardSize>::SIZE {
                trim_whitespaces(&mut input);
                let cell_value = match input.next() {
                    Some('_') => None,
                    Some('○') => Some(Player::Black),
                    Some('●') => Some(Player::White),
                    char => {
                        return Err(format!(
                            "Invalid input format: expected '○' for black, '●' for white, or ' ' for empty cell but got {char:?}",
                        ));
                    }
                };
                board.set(Pos::from_xy(x, y), cell_value);
            }
            trim_whitespaces(&mut input);
        }
        trim_whitespaces(&mut input);
        if let Some(char) = input.next() {
            return Err(format!(
                "Invalid input format: extra characters found after board: {char:?}"
            ));
        }
        Ok(board)
    }
}

impl<BS: BoardSize> Index<Pos<BS>> for Board<BS>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
{
    type Output = Option<Player>;

    #[inline]
    fn index(&self, pos: Pos<BS>) -> &Self::Output {
        let index = Self::index(pos);
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

fn trim_whitespaces(input: &mut std::iter::Peekable<std::str::Chars>) {
    while let Some(&c) = input.peek() {
        if c.is_whitespace() {
            input.next(); // consume whitespace
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{BoardSize9x9, BoardSize13x13, BoardSize19x19, Player};

    use super::*;

    #[test]
    fn memory_size() {
        assert_eq!(96, std::mem::size_of::<Board<BoardSize19x19>>());
        assert_eq!(48, std::mem::size_of::<Board<BoardSize13x13>>());
        assert_eq!(24, std::mem::size_of::<Board<BoardSize9x9>>());
    }

    #[test]
    fn empty_board() {
        let board = Board::<BoardSize13x13>::new();
        for y in 0..13 {
            for x in 0..13 {
                assert_eq!(
                    board[Pos::from_xy(x, y)],
                    None,
                    "Cell ({}, {}) should be empty",
                    x,
                    y
                );
                assert!(
                    !board.is_occupied(Pos::from_xy(x, y)),
                    "Cell ({}, {}) should not be occupied",
                    x,
                    y,
                );
            }
        }
    }

    #[test]
    fn set_and_get_cells() {
        let mut board = Board::<BoardSize13x13>::new();

        board.set(Pos::from_xy(0, 0), Some(Player::White));
        assert_eq!(board[Pos::from_xy(0, 0)], Some(Player::White));

        board.set(Pos::from_xy(10, 10), Some(Player::Black));
        assert_eq!(board[Pos::from_xy(10, 10)], Some(Player::Black));

        board.set(Pos::from_xy(12, 8), Some(Player::White));
        assert_eq!(board[Pos::from_xy(12, 8)], Some(Player::White));

        for y in 0..13 {
            for x in 0..13 {
                if (x, y) != (0, 0) && (x, y) != (10, 10) && (x, y) != (12, 8) {
                    assert_eq!(
                        board[Pos::from_xy(x, y)],
                        None,
                        "Cell ({}, {}) should still be empty",
                        x,
                        y
                    );
                }
            }
        }
    }

    mod parse_board_from_string {
        use crate::board::BoardSize3x3;

        use super::*;

        #[test]
        fn test_parse_valid_board() {
            let input = r#"
                _ ○ ○
                ○ ● ●
                ○ _ ○
            "#;
            let board = Board::<BoardSize3x3>::from_str(input).unwrap();
            assert_eq!(board[Pos::from_xy(0, 0)], None);
            assert_eq!(board[Pos::from_xy(0, 1)], Some(Player::Black));
            assert_eq!(board[Pos::from_xy(0, 2)], Some(Player::Black));
            assert_eq!(board[Pos::from_xy(1, 0)], Some(Player::Black));
            assert_eq!(board[Pos::from_xy(1, 1)], Some(Player::White));
            assert_eq!(board[Pos::from_xy(1, 2)], None);
            assert_eq!(board[Pos::from_xy(2, 0)], Some(Player::Black));
            assert_eq!(board[Pos::from_xy(2, 1)], Some(Player::White));
            assert_eq!(board[Pos::from_xy(2, 2)], Some(Player::Black));
        }
    }
}
