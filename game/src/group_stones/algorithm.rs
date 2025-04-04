use crate::{Board, BoardSize, board::Pos};

use super::{grouped_stones::GroupedStones, union_find::UnionFindAlgorithm};

/// Assigns each stone on the board a number, so that connected stones have the same number.
/// Groups are consecutive numbers starting from 0, where 0 is the first group found.
pub fn group_connected_stones<BS: BoardSize>(board: &Board<BS>) -> GroupedStones<BS>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    // Using union-find algorithm

    let mut result = UnionFindAlgorithm::new();
    for current_pos in Pos::all_positions() {
        // We have already assigned groups to all rows above and in our current row to all cells to the left.
        // We now need to find the group the cell at (x, y).
        let current_stone = board[current_pos];
        let left_pos = current_pos.left();
        let top_pos = current_pos.up();
        let matches_left_stone = left_pos
            .map(|left| current_stone == board[left])
            .unwrap_or(false);
        let matches_top_stone = top_pos
            .map(|top| current_stone == board[top])
            .unwrap_or(false);
        let group_for_current = match (matches_left_stone, matches_top_stone) {
            (false, false) => {
                // No connected stones, assign a new group
                // New group points to itself
                current_pos
            }
            (true, false) => {
                // Connected to the left stone, use its group
                // TODO Would it be faster to just add it as a child below left_pos instead of looking up left_group?
                let left_group = result.find_group_root(left_pos.unwrap());
                left_group
            }
            (false, true) => {
                // Connected to the top stone, use its group
                // TODO Would it be faster to just add it as a child below top_pos instead of looking up top_group?
                let top_group = result.find_group_root(top_pos.unwrap());
                top_group
            }
            (true, true) => {
                let left_group = result.find_group_root(left_pos.unwrap());
                let top_group = result.find_group_root(top_pos.unwrap());
                if left_group == top_group {
                    // Both left and top are in the same group, just add ourselves to it
                    left_group
                } else {
                    // The new stone connects left and top. Merge the groups.
                    let surviving_group_root = result.merge_groups(left_group, top_group);
                    // And add ourselves to the new merged group
                    surviving_group_root
                }
            }
        };
        result.add_to_group(current_pos, group_for_current);
    }

    result.finalize()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        board::{BoardSize5x5, BoardSize7x7, parse_board_from_string},
        group_stones::group_id::GroupId,
        testutils,
    };

    use super::*;

    #[test]
    fn empty_board() {
        let board = parse_board_from_string::<BoardSize5x5>(
            r#"
            _ _ _ _ _
            _ _ _ _ _
            _ _ _ _ _
            _ _ _ _ _
            _ _ _ _ _
        "#,
        )
        .unwrap();
        let grouped = group_connected_stones(&board);
        assert_eq!(
            1,
            grouped.num_groups().into_usize(),
            "There should be 1 group: the empty spaces."
        );
        let expected_group = grouped.group_at(Pos::from_xy(0, 0));
        for y in 0..5 {
            for x in 0..5 {
                assert_eq!(expected_group, grouped.group_at(Pos::from_xy(x, y)));
            }
        }
    }

    #[test]
    fn board_filled_with_black() {
        let board = parse_board_from_string::<BoardSize5x5>(
            r#"
            ● ● ● ● ●
            ● ● ● ● ●
            ● ● ● ● ●
            ● ● ● ● ●
            ● ● ● ● ●
        "#,
        )
        .unwrap();
        let grouped = group_connected_stones(&board);
        assert_eq!(
            1,
            grouped.num_groups().into_usize(),
            "There should be 1 group: all black stones are connected."
        );
        let expected_group = grouped.group_at(Pos::from_xy(0, 0));
        for y in 0..5 {
            for x in 0..5 {
                assert_eq!(expected_group, grouped.group_at(Pos::from_xy(x, y)));
            }
        }
    }

    #[test]
    fn single_stone() {
        let board = parse_board_from_string::<BoardSize5x5>(
            r#"
            _ _ _ _ _
            _ _ _ _ _
            _ _ ○ _ _
            _ _ _ _ _
            _ _ _ _ _
        "#,
        )
        .unwrap();
        let grouped = group_connected_stones(&board);
        assert_eq!(
            2,
            grouped.num_groups().into_usize(),
            "There should be 2 groups: one for the single stone and one for the empty spaces."
        );
        let expected_group_other_stones = GroupId::from_usize(0);
        let expected_group_single_stone = GroupId::from_usize(1);
        for y in 0..5 {
            for x in 0..5 {
                if (x, y) == (2, 2) {
                    assert_eq!(
                        expected_group_single_stone,
                        grouped.group_at(Pos::from_xy(x, y))
                    );
                } else {
                    assert_eq!(
                        expected_group_other_stones,
                        grouped.group_at(Pos::from_xy(x, y))
                    );
                }
            }
        }
    }

    fn parse_groups_from_string<BS: BoardSize>(input: &str) -> Result<GroupedStones<BS>, String>
    where
        [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
    {
        let mut seen_groups = HashSet::new();
        let mut parser = testutils::NumbersParser::new(input);
        let mut groups = [GroupId::ZERO; <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE];
        let mut num_groups = GroupId::ZERO;
        for i in 0..(<BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE) {
            let group =
                GroupId::from_usize(usize::try_from(parser.next_number().unwrap()).unwrap());
            groups[i] = group;
            if seen_groups.insert(group) {
                num_groups.increment();
            }
        }
        assert!(parser.next_number().is_none());
        Ok(GroupedStones::new(groups, num_groups))
    }

    fn assert_groups_eq<BS: BoardSize>(grouped: &GroupedStones<BS>, expected_groups_str: &str)
    where
        [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
    {
        let expected_groups = parse_groups_from_string::<BS>(expected_groups_str)
            .expect("Failed to parse expected groups from string");
        assert_eq!(
            expected_groups, *grouped,
            "The grouped stones do not match the expected groups.\nExpected:\n{:?}\nGot:\n{:?}",
            expected_groups, grouped,
        );
    }

    #[test]
    fn more_complicated_board() {
        let board = parse_board_from_string::<BoardSize5x5>(
            r#"
            _ ● _ ○ ○
            ● ● ○ _ _
            _ ○ ○ ● _
            ○ _ _ ● _
            _ _ _ _ ○
        "#,
        )
        .unwrap();
        let grouped = group_connected_stones(&board);

        assert_groups_eq(
            &grouped,
            r#"
            0 1 2 3 3
            1 1 4 5 5
            6 4 4 7 5
            8 9 9 7 5
            9 9 9 9 10
        "#,
        );
    }

    #[test]
    fn test_merging_groups() {
        // The algorithm goes top-bottom and each row left-right. Let's test a scenario where that causes it to first assign different groups
        // but later merge them when it finds a connection.
        let board = parse_board_from_string::<BoardSize7x7>(
            // This board is crafted so no matter which direction we go, it'll have to merge groups later
            // * outer loop top-bottom or bottom-top (symmetric), inner loop left-right or right-left (symmetric): black stones will have to be merged
            // * outer loop left-right or right-left (symmetric), inner loop top-bottom or bottom-top (symmetric): white stones will have to be merged
            r#"
            ○ ● ● ● ● ● ○
            ○ _ _ ● _ _ ○
            ○ ● ● ● ● ● ○
            ○ ○ ○ ○ ○ ○ ○
            ○ ● ● ● ● ● ○
            ○ _ _ ● _ _ ○
            ○ ● ● ● ● ● ○
            "#,
        )
        .unwrap();

        let grouped = group_connected_stones(&board);

        assert_groups_eq(
            &grouped,
            r#"
            0 1 1 1 1 1 0
            0 2 2 1 3 3 0
            0 1 1 1 1 1 0
            0 0 0 0 0 0 0
            0 4 4 4 4 4 0
            0 5 5 4 6 6 0
            0 4 4 4 4 4 0
        "#,
        );
    }
}
