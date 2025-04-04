use enum_map::{EnumMap, enum_map};
use smallset::SmallSet;

use crate::{
    board::{Board, PlaceStoneError, Player},
    group_stones::{GroupedStones, group_connected_stones},
};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Game<const BOARD_SIZE: usize>
where
    [(); bitvec::mem::elts::<usize>(2 * BOARD_SIZE * BOARD_SIZE)]:,
    [(); BOARD_SIZE * BOARD_SIZE]:,
{
    board: Board<BOARD_SIZE>,
    current_player: Player,
    num_captured_by: EnumMap<Player, u8>,
}

impl<const BOARD_SIZE: usize> Game<BOARD_SIZE>
where
    [(); bitvec::mem::elts::<usize>(2 * BOARD_SIZE * BOARD_SIZE)]:,
    [(); BOARD_SIZE * BOARD_SIZE]:,
{
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::Black,
            num_captured_by: enum_map! {
                _ => 0,
            },
        }
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn board(&self) -> &Board<BOARD_SIZE> {
        &self.board
    }

    pub fn place_stone(&mut self, x: usize, y: usize) -> Result<(), PlaceStoneError> {
        self.board.set_if_empty(x, y, self.current_player)?;
        self._take_prisoners();
        self.current_player = self.current_player.other_player();

        Ok(())
    }

    fn _take_prisoners(&mut self) {
        // TODO Instead of re-calculating the union find every turn, it's probably cheaper to keep it and update it when stones are placed. Also, is then maybe a flood fill actually faster than a union find since we don't have to update the whole board when a stone is placed?
        let groups = group_connected_stones(self.board());
        let liberties_of_group = self._liberties_of_groups(&groups);
        for (group, num_liberties) in liberties_of_group.iter().enumerate() {
            if *num_liberties == 0 {
                // This group has no liberties left, so it is captured
                let num_captured = self._capture_group(&groups, u8::try_from(group).unwrap());
                self.num_captured_by[self.current_player] += num_captured;
            }
        }
    }

    fn _liberties_of_groups(&self, groups: &GroupedStones<BOARD_SIZE>) -> Vec<u8> {
        let mut liberties = vec![0; usize::from(groups.num_groups())];

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if !self.board.is_occupied(x, y) {
                    // It's an empty cell. Any neighboring group that is occupied will get a liberty added.
                    // But we need to make sure we only add it once if two neighboring fields are from the same group.
                    // This code also adds liberties to the group representing the empty cells but that doesn't really matter.
                    let mut groups_to_add_liberty_to: SmallSet<[u8; 5]> = SmallSet::new();
                    groups_to_add_liberty_to.insert(groups.group_at(x, y));
                    if x > 0 {
                        groups_to_add_liberty_to.insert(groups.group_at(x - 1, y));
                    }
                    if y > 0 {
                        groups_to_add_liberty_to.insert(groups.group_at(x, y - 1));
                    }
                    if x < BOARD_SIZE - 1 {
                        groups_to_add_liberty_to.insert(groups.group_at(x + 1, y));
                    }
                    if y < BOARD_SIZE - 1 {
                        groups_to_add_liberty_to.insert(groups.group_at(x, y + 1));
                    }
                    for group_index in groups_to_add_liberty_to.iter() {
                        liberties[usize::from(*group_index)] += 1;
                    }
                }
            }
        }

        liberties
    }

    fn _capture_group(&mut self, groups: &GroupedStones<BOARD_SIZE>, group_to_capture: u8) -> u8 {
        let mut num_captured = 0;
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if groups.group_at(x, y) == group_to_capture {
                    // This stone is part of the captured group, remove it from the board
                    self.board.set(x, y, None);
                    num_captured += 1;
                }
            }
        }
        num_captured
    }

    pub fn num_captured_by(&self, player: Player) -> u8 {
        self.num_captured_by[player]
    }
}

#[cfg(test)]
mod tests {
    use crate::board::parse_board_from_string;

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

    #[test]
    fn test_place_stone_and_take_prisoners() {
        let board = parse_board_from_string::<5>(
            r#"
            _ ● ○ ○ ○
            ● ● ○ ● ●
            ○ ○ ○ ● _
            ○ ● ● _ _
            _ _ _ _ ○
        "#,
        )
        .unwrap();
        let mut game = Game::<5> {
            board,
            current_player: Player::White,
            num_captured_by: enum_map! {
                Player::Black => 0,
                Player::White => 0,
            },
        };
        game.place_stone(0, 4).unwrap();
        let expected_new_board = parse_board_from_string::<5>(
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
            Game {
                board: expected_new_board,
                current_player: Player::Black,
                num_captured_by: enum_map! {
                    Player::White => 8, // White captured one group of stones
                    Player::Black => 0,
                },
            },
            game
        );
    }
}
