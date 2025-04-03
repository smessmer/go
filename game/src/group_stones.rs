use derive_more::Display;

use crate::Board;

/// Assigns each stone on the board a number, so that connected stones have the same number.
/// Groups are consecutive numbers starting from 0, where 0 is the first group found.
pub fn group_connected_stones<const BOARD_SIZE: usize>(
    board: &Board<BOARD_SIZE>,
) -> GroupedStones<BOARD_SIZE>
where
    [(); bitvec::mem::elts::<usize>(2 * BOARD_SIZE * BOARD_SIZE)]:,
    [(); BOARD_SIZE * BOARD_SIZE]:,
{
    // Using union-find algorithm

    let mut result = UnionFindAlgorithm::new();
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            // We have already assigned groups to all rows above and in our current row to all cells to the left.
            // We now need to find the group the cell at (x, y).
            let current_stone = board[(x, y)];
            let matches_left_stone = x > 0 && current_stone == board[(x - 1, y)];
            let matches_top_stone = y > 0 && current_stone == board[(x, y - 1)];
            let current_pos = Pos::from_pointed_to(x, y);
            let group_for_current = match (matches_left_stone, matches_top_stone) {
                (false, false) => {
                    // No connected stones, assign a new group
                    // New group points to itself
                    current_pos
                }
                (true, false) => {
                    // Connected to the left stone, use its group
                    // TODO Would it be faster to just add it as a child below left_pos instead of looking up left_group?
                    let left_pos = Pos::from_pointed_to(x - 1, y);
                    let left_group = result.find_group_root(left_pos);
                    left_group
                }
                (false, true) => {
                    // Connected to the top stone, use its group
                    // TODO Would it be faster to just add it as a child below top_pos instead of looking up top_group?
                    let top_pos = Pos::from_pointed_to(x, y - 1);
                    let top_group = result.find_group_root(top_pos);
                    top_group
                }
                (true, true) => {
                    let left_pos = Pos::from_pointed_to(x - 1, y);
                    let left_group = result.find_group_root(left_pos);
                    let top_pos = Pos::from_pointed_to(x, y - 1);
                    let top_group = result.find_group_root(top_pos);
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
    }

    result.finalize()
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub struct Pos<const BOARD_SIZE: usize> {
    #[display("{}")]
    index: u8,
}

impl<const BOARD_SIZE: usize> Pos<BOARD_SIZE> {
    pub fn from_pointed_to(x: usize, y: usize) -> Self {
        Self {
            index: u8::try_from(y * BOARD_SIZE + x).unwrap(),
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

pub struct GroupedStones<const BOARD_SIZE: usize>
where
    [(); BOARD_SIZE * BOARD_SIZE]:,
{
    // TODO 19x19 boards can't fit their position in a u8. Need to bump to u16 for them.
    // TODO Also other places in the code base may rely on u8 for counting or indexing stones, fix those as well. We probably should introduce a typedef that depends on BOARD_SIZE. Or maybe centralize the [Pos] class from here and make it do that.
    groups: [u8; BOARD_SIZE * BOARD_SIZE],

    num_groups: u8,
}

impl<const BOARD_SIZE: usize> GroupedStones<BOARD_SIZE>
where
    [(); BOARD_SIZE * BOARD_SIZE]:,
{
    pub fn new(groups: [u8; BOARD_SIZE * BOARD_SIZE], num_groups: u8) -> Self {
        Self { groups, num_groups }
    }

    pub fn group_at(&self, x: usize, y: usize) -> u8 {
        assert!(
            x < BOARD_SIZE && y < BOARD_SIZE,
            "Coordinates out of bounds"
        );
        self.groups[y * BOARD_SIZE + x]
    }

    pub fn num_groups(&self) -> u8 {
        self.num_groups
    }
}

struct UnionFindAlgorithm<const BOARD_SIZE: usize>
where
    [(); BOARD_SIZE * BOARD_SIZE]:,
{
    // Nodes pointing to themselves are roots and representatives of their group.
    // Invariant A: forall i: groups[i].index() <= i (i.e. each node points to a parent that is either further up, or if in the same row then to the left, or itself)
    groups: [Pos<BOARD_SIZE>; BOARD_SIZE * BOARD_SIZE],
}

impl<const BOARD_SIZE: usize> UnionFindAlgorithm<BOARD_SIZE>
where
    [(); BOARD_SIZE * BOARD_SIZE]:,
{
    pub fn new() -> Self {
        Self {
            // Initial state is all nodes belong to the same group.
            // Doesn't matter though because we never read any of those before writing to it.
            groups: [Pos::from_pointed_to(0, 0); BOARD_SIZE * BOARD_SIZE],
        }
    }

    pub fn add_to_group(&mut self, pos: Pos<BOARD_SIZE>, group_root: Pos<BOARD_SIZE>) {
        assert!(group_root.index <= pos.index, "Invariant A violated");
        self.groups[pos.index()] = group_root;
    }

    pub fn find_group_root(&mut self, current_pos: Pos<BOARD_SIZE>) -> Pos<BOARD_SIZE> {
        let mut current_pos = current_pos;
        let mut parent_pos = self.groups[current_pos.index()];
        let mut grandparent_pos = self.groups[parent_pos.index()];
        while parent_pos != current_pos {
            // current_pos is not the root yet

            // Path compression using path splitting - replace the parent at the current position with the grand parent
            self.groups[current_pos.index()] = grandparent_pos; // Invariant A is upheld because of transitivity of the <= operator

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
        lhs_group_root: Pos<BOARD_SIZE>,
        rhs_group_root: Pos<BOARD_SIZE>,
    ) -> Pos<BOARD_SIZE> {
        // union-by-size or union-by-rank would be more efficient.
        // But we need to uphold invariant A so we don't have a choice.
        if lhs_group_root.index <= rhs_group_root.index {
            self.groups[rhs_group_root.index()] = lhs_group_root;
            lhs_group_root
        } else {
            self.groups[lhs_group_root.index()] = rhs_group_root;
            rhs_group_root
        }
    }

    pub fn finalize(&mut self) -> GroupedStones<BOARD_SIZE> {
        let mut groups = [0u8; BOARD_SIZE * BOARD_SIZE];
        // Because of invariant A, we know we'll always see the group root before seeing any other members of the group.
        // This means to get consecutive group numbers, we can just assign each root a new number, and for non-roots
        // we know we've already assigned a number to the root and can look it up.
        let mut current_group_number = 0;
        for index in 0..(BOARD_SIZE * BOARD_SIZE) {
            let current_pos = Pos::from_index(index);
            let root_of_current_group = self.find_group_root(current_pos);
            if root_of_current_group == current_pos {
                // This is a root, assign it a new group number
                groups[index] = current_group_number;
                current_group_number += 1;
            } else {
                // Not a root, find its group number from the root
                groups[index] = groups[root_of_current_group.index()];
            }
        }
        GroupedStones::new(groups, current_group_number)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
        assert_eq!(
            1,
            grouped.num_groups(),
            "There should be 1 group: the empty spaces."
        );
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
        assert_eq!(
            1,
            grouped.num_groups(),
            "There should be 1 group: all black stones are connected."
        );
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
        assert_eq!(
            2,
            grouped.num_groups(),
            "There should be 2 groups: one for the single stone and one for the empty spaces."
        );
        let expected_group_other_stones = 0;
        let expected_group_single_stone = 1;
        for y in 0..5 {
            for x in 0..5 {
                if (x, y) == (2, 2) {
                    assert_eq!(expected_group_single_stone, grouped.group_at(x, y));
                } else {
                    assert_eq!(expected_group_other_stones, grouped.group_at(x, y));
                }
            }
        }
    }

    fn parse_groups_from_string<const BOARD_SIZE: usize>(
        input: &str,
    ) -> Result<GroupedStones<BOARD_SIZE>, String>
    where
        [(); BOARD_SIZE * BOARD_SIZE]:,
    {
        let mut seen_groups = HashSet::new();
        let mut parser = testutils::NumbersParser::new(input);
        let mut groups = [0u8; BOARD_SIZE * BOARD_SIZE];
        let mut num_groups = 0;
        for i in 0..(BOARD_SIZE * BOARD_SIZE) {
            let group = u8::try_from(parser.next_number().unwrap()).unwrap();
            groups[i] = group;
            if seen_groups.insert(group) {
                num_groups += 1;
            }
        }
        assert!(parser.next_number().is_none());
        Ok(GroupedStones::new(groups, num_groups))
    }

    fn assert_groups_eq<const BOARD_SIZE: usize>(
        grouped: &GroupedStones<BOARD_SIZE>,
        expected_groups_str: &str,
    ) where
        [(); BOARD_SIZE * BOARD_SIZE]:,
    {
        let expected_groups = parse_groups_from_string::<BOARD_SIZE>(expected_groups_str)
            .expect("Failed to parse expected groups from string");
        assert_eq!(
            &expected_groups.groups, &grouped.groups,
            "The grouped stones do not match the expected groups.\nExpected:\n{:?}\nGot:\n{:?}",
            expected_groups.groups, grouped.groups
        );
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
