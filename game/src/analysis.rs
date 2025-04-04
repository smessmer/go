use derive_where::derive_where;
use smallset::SmallSet;

use crate::{
    Board, BoardSize, NumStones, Player,
    board::Pos,
    group_stones::{GroupId, GroupedStones, group_connected_stones},
};

#[derive_where(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupInfo<BS: BoardSize> {
    pub owner: Option<Player>,
    pub liberties: NumStones<BS>,
}

/// Analyses a board position, determining groups, liberties, and other properties.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Analysis<BS: BoardSize>
where
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
{
    /// Mapping from board position to which group it belongs to
    pos_to_group: GroupedStones<BS>,

    /// Some info for each group
    group_info: Vec<GroupInfo<BS>>,
}

impl<BS: BoardSize> Analysis<BS>
where
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
{
    pub fn analyze(board: &Board<BS>) -> Self {
        let pos_to_group = group_connected_stones(board);
        let group_info = Self::_liberties_and_owners_of_groups(board, &pos_to_group);
        Self {
            pos_to_group,
            group_info,
        }
    }

    /// Update group info, assuming group info may have changed but the positioning of groups hasn't changed,
    /// i.e. it's still an isomorphic layout and each two stones that previously were in same/different groups
    /// are still in same/different groups.
    /// This can be called after prisoners have been captured for example but cannot be called if stones have been placed
    /// because placing stones can change group isomorphy and will require a full update.
    pub fn update_group_info(&mut self, board: &Board<BS>) {
        self.group_info = Self::_liberties_and_owners_of_groups(board, &self.pos_to_group);
    }

    pub fn group_at(&self, pos: Pos<BS>) -> GroupId<BS> {
        self.pos_to_group.group_at(pos)
    }

    fn _liberties_and_owners_of_groups(
        board: &Board<BS>,
        pos_to_group: &GroupedStones<BS>,
    ) -> Vec<GroupInfo<BS>> {
        let mut liberties_and_owners = vec![
            GroupInfo {
                owner: None,
                liberties: NumStones::ZERO
            };
            pos_to_group.num_groups().into_usize()
        ];

        for y in 0..<BS as BoardSize>::SIZE {
            for x in 0..<BS as BoardSize>::SIZE {
                if board.is_occupied(Pos::from_xy(x, y)) {
                    // It's a filled cell. Remember the owner of this group
                    let group = pos_to_group.group_at(Pos::from_xy(x, y)).into_usize();
                    if liberties_and_owners[group].owner.is_none() {
                        liberties_and_owners[group].owner = board[Pos::from_xy(x, y)];
                    }
                } else {
                    // It's an empty cell. Any neighboring group that is occupied will get a liberty added.
                    // But we need to make sure we only add it once if two neighboring fields are from the same group.
                    // This code also adds liberties to the group representing the empty cells but that doesn't really matter.
                    let mut groups_to_add_liberty_to: SmallSet<[GroupId<BS>; 5]> = SmallSet::new();
                    groups_to_add_liberty_to.insert(pos_to_group.group_at(Pos::from_xy(x, y)));
                    if x > 0 {
                        groups_to_add_liberty_to
                            .insert(pos_to_group.group_at(Pos::from_xy(x - 1, y)));
                    }
                    if y > 0 {
                        groups_to_add_liberty_to
                            .insert(pos_to_group.group_at(Pos::from_xy(x, y - 1)));
                    }
                    if x < <BS as BoardSize>::SIZE - 1 {
                        groups_to_add_liberty_to
                            .insert(pos_to_group.group_at(Pos::from_xy(x + 1, y)));
                    }
                    if y < <BS as BoardSize>::SIZE - 1 {
                        groups_to_add_liberty_to
                            .insert(pos_to_group.group_at(Pos::from_xy(x, y + 1)));
                    }
                    for group_index in groups_to_add_liberty_to.iter() {
                        liberties_and_owners[group_index.into_usize()].liberties += NumStones::ONE;
                    }
                }
            }
        }

        liberties_and_owners
    }

    pub fn groups(
        &self,
    ) -> impl Iterator<Item = (GroupId<BS>, &GroupInfo<BS>)> + ExactSizeIterator + use<'_, BS> {
        self.group_info
            .iter()
            .enumerate()
            .map(|(index, info)| (GroupId::from_usize(index), info))
    }
}
