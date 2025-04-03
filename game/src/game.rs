use crate::board::{Board, PlaceStoneError, Player};

pub struct Game<const BoardSize: usize>
where
    [(); bitvec::mem::elts::<usize>(2 * BoardSize * BoardSize)]:,
{
    board: Board<BoardSize>,
    current_player: Player,
}

impl<const BoardSize: usize> Game<BoardSize>
where
    [(); bitvec::mem::elts::<usize>(2 * BoardSize * BoardSize)]:,
{
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::Black,
        }
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn board(&self) -> &Board<BoardSize> {
        &self.board
    }

    pub fn place_stone(&mut self, x: usize, y: usize) -> Result<(), PlaceStoneError> {
        self.board.set_if_empty(x, y, self.current_player)?;
        self.current_player = self.current_player.other_player();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_initial_state() {
        let game = Game::<13>::new();
        assert_eq!(game.current_player(), Player::Black);
        assert_eq!(
            game.board()
                .iter()
                .filter(|(_x, _y, cell)| cell.is_some())
                .count(),
            0
        );
    }

    #[test]
    fn test_place_stone_success() {
        let mut game = Game::<13>::new();
        assert!(game.place_stone(10, 5).is_ok());
        assert_eq!(game.board()[(10, 5)], Some(Player::Black));
        assert_eq!(game.current_player(), Player::White);
    }

    #[test]
    fn test_place_stone_on_occupied_space() {
        let mut game = Game::<13>::new();
        assert!(game.place_stone(10, 5).is_ok());
        assert_eq!(game.board()[(10, 5)], Some(Player::Black));
        assert_eq!(game.current_player(), Player::White);

        assert!(game.place_stone(10, 5).is_err());
        assert_eq!(game.board()[(10, 5)], Some(Player::Black));
        assert_eq!(game.current_player(), Player::White);
    }

    #[test]
    fn test_alternating_players() {
        let mut game = Game::<13>::new();
        assert!(game.place_stone(0, 0).is_ok());
        assert_eq!(game.current_player(), Player::White);
        assert!(game.place_stone(1, 1).is_ok());
        assert_eq!(game.current_player(), Player::Black);
    }
}
