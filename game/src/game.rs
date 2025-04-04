use enum_map::{EnumMap, enum_map};
use smallset::SmallSet;

use crate::{
    NumStones,
    board::{Board, BoardSize, PlaceStoneError, Player},
    group_stones::{GroupId, GroupedStones, group_connected_stones},
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
}

impl<BS: BoardSize> Game<BS>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::Black,
            num_captured_by: enum_map! {
                _ => NumStones::from_usize(0),
            },
        }
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn board(&self) -> &Board<BS> {
        &self.board
    }

    pub fn place_stone(&mut self, x: usize, y: usize) -> Result<(), PlaceStoneError> {
        self.board.set_if_empty(x, y, self.current_player)?;
        self._take_prisoners();
        self.current_player = self.current_player.other_player();

        Ok(())
    }

    pub fn pass_turn(&mut self) {
        self.current_player = self.current_player.other_player();
        // No need to take prisoners or update the board since no stone was placed
    }

    fn _take_prisoners(&mut self) {
        // TODO Instead of re-calculating the union find every turn, it's probably cheaper to keep it and update it when stones are placed. Also, is then maybe a flood fill actually faster than a union find since we don't have to update the whole board when a stone is placed?
        let groups = group_connected_stones(self.board());
        let liberties_of_group = self._liberties_of_groups(&groups);
        for (group, num_liberties) in liberties_of_group.iter().enumerate() {
            if *num_liberties == NumStones::ZERO {
                // This group has no liberties left, so it is captured
                let num_captured = self._capture_group(&groups, GroupId::from_usize(group));
                self.num_captured_by[self.current_player] += num_captured;
            }
        }
    }

    fn _liberties_of_groups(&self, groups: &GroupedStones<BS>) -> Vec<NumStones<BS>> {
        let mut liberties = vec![NumStones::ZERO; groups.num_groups().into_usize()];

        for y in 0..<BS as BoardSize>::SIZE {
            for x in 0..<BS as BoardSize>::SIZE {
                if !self.board.is_occupied(x, y) {
                    // It's an empty cell. Any neighboring group that is occupied will get a liberty added.
                    // But we need to make sure we only add it once if two neighboring fields are from the same group.
                    // This code also adds liberties to the group representing the empty cells but that doesn't really matter.
                    let mut groups_to_add_liberty_to: SmallSet<[GroupId<BS>; 5]> = SmallSet::new();
                    groups_to_add_liberty_to.insert(groups.group_at(x, y));
                    if x > 0 {
                        groups_to_add_liberty_to.insert(groups.group_at(x - 1, y));
                    }
                    if y > 0 {
                        groups_to_add_liberty_to.insert(groups.group_at(x, y - 1));
                    }
                    if x < <BS as BoardSize>::SIZE - 1 {
                        groups_to_add_liberty_to.insert(groups.group_at(x + 1, y));
                    }
                    if y < <BS as BoardSize>::SIZE - 1 {
                        groups_to_add_liberty_to.insert(groups.group_at(x, y + 1));
                    }
                    for group_index in groups_to_add_liberty_to.iter() {
                        liberties[group_index.into_usize()] += NumStones::ONE;
                    }
                }
            }
        }

        liberties
    }

    fn _capture_group(
        &mut self,
        groups: &GroupedStones<BS>,
        group_to_capture: GroupId<BS>,
    ) -> NumStones<BS> {
        let mut num_captured = NumStones::ZERO;
        for y in 0..<BS as BoardSize>::SIZE {
            for x in 0..<BS as BoardSize>::SIZE {
                if groups.group_at(x, y) == group_to_capture {
                    // This stone is part of the captured group, remove it from the board
                    self.board.set(x, y, None);
                    num_captured += NumStones::ONE;
                }
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
    use crate::board::{BoardSize5x5, BoardSize13x13, parse_board_from_string};

    use super::*;

    #[test]
    fn test_new_game_initial_state() {
        let game = Game::<BoardSize13x13>::new();
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
        let mut game = Game::<BoardSize13x13>::new();
        assert!(game.place_stone(10, 5).is_ok());
        assert_eq!(game.board()[(10, 5)], Some(Player::Black));
        assert_eq!(game.current_player(), Player::White);
    }

    #[test]
    fn test_place_stone_on_occupied_space() {
        let mut game = Game::<BoardSize13x13>::new();
        assert!(game.place_stone(10, 5).is_ok());
        assert_eq!(game.board()[(10, 5)], Some(Player::Black));
        assert_eq!(game.current_player(), Player::White);

        assert!(game.place_stone(10, 5).is_err());
        assert_eq!(game.board()[(10, 5)], Some(Player::Black));
        assert_eq!(game.current_player(), Player::White);
    }

    #[test]
    fn test_alternating_players() {
        let mut game = Game::<BoardSize13x13>::new();
        assert!(game.place_stone(0, 0).is_ok());
        assert_eq!(game.current_player(), Player::White);
        assert!(game.place_stone(1, 1).is_ok());
        assert_eq!(game.current_player(), Player::Black);
    }

    #[test]
    fn test_place_stone_and_take_prisoners() {
        let board = parse_board_from_string::<BoardSize5x5>(
            r#"
            _ ● ○ ○ ○
            ● ● ○ ● ●
            ○ ○ ○ ● _
            ○ ● ● _ _
            _ _ _ _ ○
        "#,
        )
        .unwrap();
        let mut game = Game::<BoardSize5x5> {
            board,
            current_player: Player::White,
            num_captured_by: enum_map! {
                Player::Black => NumStones::ZERO,
                Player::White => NumStones::ZERO,
            },
        };
        game.place_stone(0, 4).unwrap();
        let expected_new_board = parse_board_from_string::<BoardSize5x5>(
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
                    Player::White => NumStones::from_usize(8), // White captured one group of stones
                    Player::Black => NumStones::from_usize(0),
                },
            },
            game
        );
    }
}
