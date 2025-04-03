use derive_more::Display;

use crate::Board;

/// Assigns each stone on the board a number, so that connected stones have the same number.
/// Numbers are not necessarily consecutive, there can be gaps.
///
/// Result is indexed as result[y][x]
pub fn group_connected_stones<const BoardSize: usize>(
    board: &Board<BoardSize>,
) -> GroupedStones<BoardSize>
where
    [(); bitvec::mem::elts::<usize>(2 * BoardSize * BoardSize)]:,
    [(); BoardSize * BoardSize]:,
{
    // Using union-find algorithm

    let mut result = UnionFindAlgorithm::new();
    for y in 0..BoardSize {
        for x in 0..BoardSize {
            // We have already assigned groups to all rows above and in our current row to all cells to the left.
            // We now need to find the group the cell at (x, y).
            let current_stone = board[(x, y)];
            let matches_left_stone = x > 0 && current_stone == board[(x - 1, y)];
            let matches_top_stone = y > 0 && current_stone == board[(x, y - 1)];
            let current_pos = Pos::from_pointed_to(x, y);
            match (matches_left_stone, matches_top_stone) {
                (false, false) => {
                    // No connected stones, assign a new group
                    result.add_new_group_at(current_pos);
                }
                (true, false) => {
                    // Connected to the left stone, use its group
                    let left_pos = Pos::from_pointed_to(x - 1, y);
                    let left_group = result.find_group_root(left_pos);
                    result.add_to_group(current_pos, left_group);
                }
                (false, true) => {
                    // Connected to the top stone, use its group
                    let top_pos = Pos::from_pointed_to(x, y - 1);
                    let top_group = result.find_group_root(top_pos);
                    result.add_to_group(current_pos, top_group);
                }
                (true, true) => {
                    let left_pos = Pos::from_pointed_to(x - 1, y);
                    let left_group = result.find_group_root(left_pos);
                    let top_pos = Pos::from_pointed_to(x, y - 1);
                    let top_group = result.find_group_root(top_pos);
                    let group_for_current = if left_group == top_group {
                        // Both left and top are in the same group, just add ourselves to it
                        left_group
                    } else {
                        // The new stone connects left and top. Merge the groups.
                        let surviving_group_root = result.merge_groups(left_group, top_group);
                        // And add ourselves to the new merged group
                        surviving_group_root
                    };
                    result.add_to_group(current_pos, group_for_current);
                }
            }
        }
    }

    result.finalize()
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub struct Pos<const BoardSize: usize> {
    #[display("{}")]
    index: u8,
}

impl<const BoardSize: usize> Pos<BoardSize> {
    pub fn from_pointed_to(x: usize, y: usize) -> Self {
        Self {
            index: u8::try_from(y * BoardSize + x).unwrap(),
        }
    }

    pub fn from_index(index: usize) -> Self {
        Self {
            index: u8::try_from(index).unwrap(),
        }
    }

    pub fn index(&self) -> usize {
        usize::from(self.index)
    }
}

pub struct GroupedStones<const BoardSize: usize>
where
    [(); BoardSize * BoardSize]:,
{
    // TODO 19x19 boards can't fit their position in a u8. Need to bump to u16 for them.
    groups: [u8; BoardSize * BoardSize],
}

impl<const BoardSize: usize> GroupedStones<BoardSize>
where
    [(); BoardSize * BoardSize]:,
{
    pub fn new(groups: [u8; BoardSize * BoardSize]) -> Self {
        Self { groups }
    }

    pub fn group_at(&self, x: usize, y: usize) -> u8 {
        assert!(x < BoardSize && y < BoardSize, "Coordinates out of bounds");
        self.groups[y * BoardSize + x]
    }
}

struct UnionFindAlgorithm<const BoardSize: usize>
where
    [(); BoardSize * BoardSize]:,
{
    // Nodes pointing to themselves are roots and representatives of their group.
    groups: [Pos<BoardSize>; BoardSize * BoardSize],
}

impl<const BoardSize: usize> UnionFindAlgorithm<BoardSize>
where
    [(); BoardSize * BoardSize]:,
{
    pub fn new() -> Self {
        Self {
            // Initial state is all nodes belong to the same group.
            // Doesn't matter though because we never read any of those before writing to it.
            groups: [Pos::from_pointed_to(0, 0); BoardSize * BoardSize],
        }
    }

    pub fn add_new_group_at(&mut self, pos: Pos<BoardSize>) {
        // New group points to itself
        self.add_to_group(pos, pos);
    }

    pub fn add_to_group(&mut self, pos: Pos<BoardSize>, group_root: Pos<BoardSize>) {
        self.groups[pos.index()] = group_root;
    }

    pub fn find_group_root(&mut self, current_pos: Pos<BoardSize>) -> Pos<BoardSize> {
        let mut current_pos = current_pos;
        let mut parent_pos = self.groups[current_pos.index()];
        let mut grandparent_pos = self.groups[parent_pos.index()];
        while parent_pos != current_pos {
            // current_pos is not the root yet

            // Path compression using path splitting - replace the parent at the current position with the grand parent
            self.groups[current_pos.index()] = grandparent_pos;

            // And move one closer to the root
            current_pos = parent_pos;
            parent_pos = grandparent_pos;
            grandparent_pos = self.groups[parent_pos.index()];
        }
        current_pos
    }

    /// Merge the two groups given their roots, returning the root of the merged group.
    pub fn merge_groups(
        &mut self,
        lhs_group_root: Pos<BoardSize>,
        rhs_group_root: Pos<BoardSize>,
    ) -> Pos<BoardSize> {
        // For simplicity, we always make the left group the root.
        // TODO union-by-size or union-by-rank would be more efficient
        self.groups[rhs_group_root.index()] = lhs_group_root;
        lhs_group_root
    }

    pub fn finalize(&mut self) -> GroupedStones<BoardSize> {
        let mut groups = [0u8; BoardSize * BoardSize];
        for index in 0..(BoardSize * BoardSize) {
            groups[index] = self.find_group_root(Pos::from_index(index)).index;
        }
        GroupedStones::new(groups)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, hash_map::Entry};

    use crate::{board::parse_board_from_string, testutils};

    use super::*;

    #[test]
    fn empty_board() {
        let board = parse_board_from_string::<5>(
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
        let expected_group = grouped.group_at(0, 0);
        for y in 0..5 {
            for x in 0..5 {
                assert_eq!(expected_group, grouped.group_at(x, y));
            }
        }
    }

    #[test]
    fn board_filled_with_black() {
        let board = parse_board_from_string::<5>(
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
        let expected_group = grouped.group_at(0, 0);
        for y in 0..5 {
            for x in 0..5 {
                assert_eq!(expected_group, grouped.group_at(x, y));
            }
        }
    }

    #[test]
    fn single_stone() {
        let board = parse_board_from_string::<5>(
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
        let expected_group_other_stones = grouped.group_at(0, 0);
        assert_ne!(expected_group_other_stones, grouped.group_at(2, 2)); // The single stone is in its own group
        for y in 0..5 {
            for x in 0..5 {
                if (x, y) != (2, 2) {
                    assert_eq!(expected_group_other_stones, grouped.group_at(x, y)); // All other cells should be in a shared group
                }
            }
        }
    }

    fn parse_groups_from_string<const BoardSize: usize>(
        input: &str,
    ) -> Result<GroupedStones<BoardSize>, String>
    where
        [(); BoardSize * BoardSize]:,
    {
        let mut parser = testutils::NumbersParser::new(input);
        let mut groups = [0u8; BoardSize * BoardSize];
        for i in 0..(BoardSize * BoardSize) {
            groups[i] = u8::try_from(parser.next_number().unwrap()).unwrap();
        }
        assert!(parser.next_number().is_none());
        Ok(GroupedStones::new(groups))
    }

    fn assert_groups_isomorphic<const BoardSize: usize>(
        lhs: &GroupedStones<BoardSize>,
        rhs: &GroupedStones<BoardSize>,
    ) where
        [(); BoardSize * BoardSize]:,
    {
        // Assert that each group in lhs can be mapped to a unique group in rhs
        fn assert_is_injective<const BoardSize: usize>(
            lhs: &GroupedStones<BoardSize>,
            rhs: &GroupedStones<BoardSize>,
        ) where
            [(); BoardSize * BoardSize]:,
        {
            // Groups can have different number in `lhs` and `rhs`, we just assert that the same elements are grouped together.
            // We need to remember a mapping between the group numbers in lhs and rhs.
            let mut map_lhs_to_rhs = HashMap::new();
            for y in 0..BoardSize {
                for x in 0..BoardSize {
                    let lhs_group = lhs.group_at(x, y);
                    let rhs_group = rhs.group_at(x, y);
                    match map_lhs_to_rhs.entry(lhs_group) {
                        Entry::Vacant(entry) => {
                            entry.insert(rhs_group);
                        }
                        Entry::Occupied(entry) => {
                            assert_eq!(
                                *entry.get(),
                                rhs_group,
                                "Groups at ({x}, {y}) do not match",
                            );
                        }
                    }
                }
            }
        }

        // If lhs->rhs is injective and rhs->lhs is injective, then the mapping is bijective, which means lhs and rhs are isomorphic
        assert_is_injective(lhs, rhs);
        assert_is_injective(rhs, lhs);
    }

    fn assert_has_groups<const BoardSize: usize>(
        grouped: &GroupedStones<BoardSize>,
        expected_groups_str: &str,
    ) where
        [(); BoardSize * BoardSize]:,
    {
        let expected_groups = parse_groups_from_string::<BoardSize>(expected_groups_str)
            .expect("Failed to parse expected groups from string");
        assert_groups_isomorphic(&expected_groups, grouped);
    }

    #[test]
    fn more_complicated_board() {
        let board = parse_board_from_string::<5>(
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

        assert_has_groups(
            &grouped,
            r#"
            01 11 02 12 12
            11 11 13 03 03
            04 13 13 15 03
            14 05 05 15 03
            05 05 05 05 16
        "#,
        );
    }

    #[test]
    fn test_merging_groups() {
        // The algorithm goes top-bottom and each row left-right. Let's test a scenario where that causes it to first assign different groups
        // but later merge them when it finds a connection.
        let board = parse_board_from_string::<7>(
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

        assert_has_groups(
            &grouped,
            r#"
            5 6 6 6 6 6 5
            5 0 0 6 1 1 5
            5 6 6 6 6 6 5
            5 5 5 5 5 5 5
            5 7 7 7 7 7 5
            5 2 2 7 3 3 5
            5 7 7 7 7 7 5
        "#,
        );
    }
}
