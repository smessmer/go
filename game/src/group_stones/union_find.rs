use crate::board::{BoardSize, Pos};

use super::{GroupId, GroupedStones};

pub struct UnionFindAlgorithm<BS: BoardSize>
where
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    // Nodes pointing to themselves are roots and representatives of their group.
    // Invariant A: forall i: groups[i].index() <= i (i.e. each node points to a parent that is either further up, or if in the same row then to the left, or itself)
    groups: [Pos<BS>; <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE],
}

impl<BS: BoardSize> UnionFindAlgorithm<BS>
where
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    pub fn new() -> Self {
        Self {
            // Initial state is all nodes belong to the same group.
            // Doesn't matter though because we never read any of those before writing to it.
            groups: [Pos::from_pointed_to(0, 0); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE],
        }
    }

    pub fn add_to_group(&mut self, pos: Pos<BS>, group_root: Pos<BS>) {
        assert!(group_root <= pos, "Invariant A violated");
        self.groups[pos.index()] = group_root;
    }

    pub fn find_group_root(&mut self, current_pos: Pos<BS>) -> Pos<BS> {
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
    pub fn merge_groups(&mut self, lhs_group_root: Pos<BS>, rhs_group_root: Pos<BS>) -> Pos<BS> {
        // union-by-size or union-by-rank would be more efficient.
        // But we need to uphold invariant A so we don't have a choice.
        if lhs_group_root <= rhs_group_root {
            self.groups[rhs_group_root.index()] = lhs_group_root;
            lhs_group_root
        } else {
            self.groups[lhs_group_root.index()] = rhs_group_root;
            rhs_group_root
        }
    }

    pub fn finalize(&mut self) -> GroupedStones<BS> {
        let mut groups = [GroupId::ZERO; <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE];
        // Because of invariant A, we know we'll always see the group root before seeing any other members of the group.
        // This means to get consecutive group numbers, we can just assign each root a new number, and for non-roots
        // we know we've already assigned a number to the root and can look it up.
        let mut current_group_number = GroupId::ZERO;
        for index in 0..(<BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE) {
            let current_pos = Pos::from_index(index);
            let root_of_current_group = self.find_group_root(current_pos);
            if root_of_current_group == current_pos {
                // This is a root, assign it a new group number
                groups[index] = current_group_number;
                current_group_number.increment();
            } else {
                // Not a root, find its group number from the root
                groups[index] = groups[root_of_current_group.index()];
            }
        }
        GroupedStones::new(groups, current_group_number)
    }
}
