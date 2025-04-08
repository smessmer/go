use enum_map::{EnumMap, enum_map};

use crate::{
    NumStones,
    analysis::{Analysis, GroupInfo},
    board::{Board, BoardSize, PlaceStoneError, Player, Pos},
    group_stones::GroupId,
};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Game<BS: BoardSize>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    board: Board<BS>,
    current_player: Player,
    num_captured_by: EnumMap<Player, NumStones<BS>>,
    analysis: Analysis<BS>,
}

impl<BS: BoardSize> Game<BS>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    pub fn new() -> Self {
        let board = Board::new();
        let analysis = Analysis::analyze(&board);
        Self {
            board,
            current_player: Player::Black,
            num_captured_by: enum_map! {
                _ => NumStones::from_usize(0),
            },
            analysis,
        }
    }

    #[cfg(test)]
    pub fn from_board(
        board: Board<BS>,
        current_player: Player,
        num_captured_by: EnumMap<Player, NumStones<BS>>,
    ) -> Self {
        let analysis = Analysis::analyze(&board);
        Self {
            board,
            current_player,
            num_captured_by,
            analysis,
        }
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn board(&self) -> &Board<BS> {
        &self.board
    }

    pub fn place_stone(&mut self, pos: Pos<BS>) -> Result<(), PlaceStoneError> {
        self.board.set_if_empty(pos, self.current_player)?;
        self._update_analysis();
        self._take_prisoners();
        self.current_player = self.current_player.other_player();

        Ok(())
    }

    fn _update_analysis(&mut self) {
        // TODO Instead of re-calculating the union find every turn, it's probably cheaper to keep it and update it when stones are placed. Also, is then maybe a flood fill actually faster than a union find since we don't have to update the whole board when a stone is placed?
        self.analysis = Analysis::analyze(&self.board);
    }

    pub fn pass_turn(&mut self) {
        self.current_player = self.current_player.other_player();
        // No need to take prisoners or update the board since no stone was placed
    }

    fn _take_prisoners(&mut self) {
        // First capture all opponent groups without liberties
        self._player_takes_prisoners(self.current_player);

        // Then take our own stones as prisoners
        self.analysis.update_group_info(&self.board); // TODO it would be faster to just tell the analysis what stones we took away instead of recalculating all liberties
        let opponent = self.current_player.other_player();
        self._player_takes_prisoners(opponent);
    }

    fn _player_takes_prisoners(&mut self, player: Player) {
        let opponent = player.other_player();
        let mut groups_to_capture = Vec::new();
        for (group, GroupInfo { owner, liberties }) in self.analysis.groups() {
            if *owner == Some(opponent) && *liberties == NumStones::ZERO {
                // This group has no liberties left, so it is captured
                groups_to_capture.push(group);
            }
        }
        for group in groups_to_capture {
            let num_captured = self._capture_group(group);
            self.num_captured_by[player] += num_captured;
        }
    }

    fn _capture_group(&mut self, group_to_capture: GroupId<BS>) -> NumStones<BS> {
        let mut num_captured = NumStones::ZERO;
        for pos in Pos::all_positions() {
            if self.analysis.group_at(pos) == group_to_capture {
                // This stone is part of the captured group, remove it from the board
                self.board.set(pos, None);
                num_captured += NumStones::ONE;
            }
        }
        num_captured
    }

    pub fn num_captured_by(&self, player: Player) -> NumStones<BS> {
        self.num_captured_by[player]
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{BoardSize5x5, BoardSize13x13};

    use super::*;

    #[test]
    fn test_new_game_initial_state() {
        let game = Game::<BoardSize13x13>::new();
        assert_eq!(game.current_player(), Player::Black);
        assert_eq!(
            game.board()
                .iter()
                .filter(|(_pos, cell)| cell.is_some())
                .count(),
            0
        );
    }

    #[test]
    fn test_place_stone_success() {
        let mut game = Game::<BoardSize13x13>::new();
        assert!(game.place_stone(Pos::from_xy(10, 5)).is_ok());
        assert_eq!(game.board()[Pos::from_xy(10, 5)], Some(Player::Black));
        assert_eq!(game.current_player(), Player::White);
    }

    #[test]
    fn test_place_stone_on_occupied_space() {
        let mut game = Game::<BoardSize13x13>::new();
        assert!(game.place_stone(Pos::from_xy(10, 5)).is_ok());
        assert_eq!(game.board[Pos::from_xy(10, 5)], Some(Player::Black));
        assert_eq!(game.current_player(), Player::White);

        assert!(game.place_stone(Pos::from_xy(10, 5)).is_err());
        assert_eq!(game.board()[Pos::from_xy(10, 5)], Some(Player::Black));
        assert_eq!(game.current_player(), Player::White);
    }

    #[test]
    fn test_alternating_players() {
        let mut game = Game::<BoardSize13x13>::new();
        assert!(game.place_stone(Pos::from_xy(0, 0)).is_ok());
        assert_eq!(game.current_player(), Player::White);
        assert!(game.place_stone(Pos::from_xy(1, 1)).is_ok());
        assert_eq!(game.current_player(), Player::Black);
    }

    #[test]
    fn test_place_stone_and_take_prisoners() {
        let board = Board::<BoardSize5x5>::from_str(
            r#"
            _ ● ○ ○ ○
            ● ● ○ ● ●
            ○ ○ ○ ● _
            ○ ● ● _ _
            _ _ _ _ ○
        "#,
        )
        .unwrap();
        let mut game = Game::<BoardSize5x5>::from_board(
            board,
            Player::White,
            enum_map! {
                Player::Black => NumStones::ZERO,
                Player::White => NumStones::ZERO,
            },
        );
        game.place_stone(Pos::from_xy(0, 4)).unwrap();
        let expected_new_board = Board::<BoardSize5x5>::from_str(
            r#"
            _ ● _ _ _
            ● ● _ ● ●
            _ _ _ ● _
            _ ● ● _ _
            ● _ _ _ ○
        "#,
        )
        .unwrap();
        assert_eq!(
            Game::from_board(
                expected_new_board,
                Player::Black,
                enum_map! {
                    Player::White => NumStones::from_usize(8), // White captured one group of stones
                    Player::Black => NumStones::from_usize(0),
                },
            ),
            game
        );
    }

    #[test]
    fn capture_opponent_before_capturing_self_black_moves() {
        let board = Board::<BoardSize5x5>::from_str(
            r#"
            ○ ○ ○ ○ ○
            ○ ● ● ● ○
            ○ ● _ ● ○
            ○ ● ● ● ○
            ○ ○ ○ ○ ○
        "#,
        )
        .unwrap();
        let mut game = Game::<BoardSize5x5>::from_board(
            board,
            Player::Black,
            enum_map! {
                Player::Black => NumStones::ZERO,
                Player::White => NumStones::ZERO,
            },
        );
        game.place_stone(Pos::from_xy(2, 2)).unwrap();
        let expected_new_board = Board::<BoardSize5x5>::from_str(
            r#"
            ○ ○ ○ ○ ○
            ○ _ _ _ ○
            ○ _ ○ _ ○
            ○ _ _ _ ○
            ○ ○ ○ ○ ○
        "#,
        )
        .unwrap();
        assert_eq!(
            Game::from_board(
                expected_new_board,
                Player::White,
                enum_map! {
                    Player::White => NumStones::from_usize(0),
                    Player::Black => NumStones::from_usize(8),
                },
            ),
            game
        );
    }

    #[test]
    fn capture_opponent_before_capturing_self_white_moves() {
        let board = Board::<BoardSize5x5>::from_str(
            r#"
            ● ● ● ● ●
            ● ○ ○ ○ ●
            ● ○ _ ○ ●
            ● ○ ○ ○ ●
            ● ● ● ● ●
        "#,
        )
        .unwrap();
        let mut game = Game::<BoardSize5x5>::from_board(
            board,
            Player::White,
            enum_map! {
                Player::Black => NumStones::ZERO,
                Player::White => NumStones::ZERO,
            },
        );
        game.place_stone(Pos::from_xy(2, 2)).unwrap();
        let expected_new_board = Board::<BoardSize5x5>::from_str(
            r#"
            ● ● ● ● ●
            ● _ _ _ ●
            ● _ ● _ ●
            ● _ _ _ ●
            ● ● ● ● ●
        "#,
        )
        .unwrap();
        assert_eq!(
            Game::from_board(
                expected_new_board,
                Player::Black,
                enum_map! {
                    Player::White => NumStones::from_usize(8),
                    Player::Black => NumStones::from_usize(0),
                },
            ),
            game
        );
    }
}
