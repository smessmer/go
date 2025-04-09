use derive_where::derive_where;
use smallset::SmallSet;

use crate::{
    Board, BoardSize, NumStones, Player,
    board::Pos,
    group_stones::{GroupId, GroupedStones, group_connected_stones},
};

#[derive_where(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupInfo<BS: BoardSize> {
    PlayerGroup {
        owner: Player,
        liberties: NumStones<BS>,
    },
    EmptyStonesGroup,
    // TODO Unknown can only happen while building the analysis. Is there a better way to handle this?
    Unknown {
        liberties: NumStones<BS>,
    },
}

/// Analyses a board position, determining groups, liberties, and other properties.
#[derive_where(Debug, PartialEq, Eq)]
pub struct Analysis<BS: BoardSize>
where
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
{
    /// Mapping from board position to which group it belongs to
    pos_to_group: [GroupId<BS>; <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE],

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
            pos_to_group: pos_to_group.into(),
            group_info,
        }
    }

    /// Remove a stone without splitting the group it belongs to.
    ///
    /// WARNING: This is only valid to call if the group is fully enclosed, i.e. doesn't connect to any other empty groups.
    pub fn capture_group(
        &mut self,
        group_to_capture: GroupId<BS>,
        mut on_remove: impl FnMut(Pos<BS>),
    ) {
        self.group_info[group_to_capture.into_usize()] = GroupInfo::EmptyStonesGroup;

        // TODO Would it be overall faster to keep a map of groups to positions around?
        for pos in Pos::all_positions() {
            if self.group_at(pos) == group_to_capture {
                // Remove the stone
                on_remove(pos);

                // And give each neighboring group a liberty
                for group in self.find_neighboring_groups(pos).iter() {
                    match &mut self.group_info[group.into_usize()] {
                        GroupInfo::Unknown { .. } => unreachable!(),
                        GroupInfo::PlayerGroup { liberties, .. } => *liberties += NumStones::ONE,
                        GroupInfo::EmptyStonesGroup => {
                            panic!(
                                "We captured a group that neighbors an empty group. Impossible."
                            );
                        }
                    }
                }
            }
        }
    }

    fn find_neighboring_groups(&self, pos: Pos<BS>) -> SmallSet<[GroupId<BS>; 4]> {
        let self_group = self.group_at(pos);
        let mut neighboring_groups = SmallSet::<[GroupId<BS>; 4]>::new();
        let mut check_neighbor = |neighbor_pos: Option<Pos<BS>>| {
            if let Some(neighbor) = neighbor_pos {
                let neighbor_group = self.group_at(neighbor);
                if neighbor_group != self_group {
                    neighboring_groups.insert(neighbor_group);
                }
            }
        };
        check_neighbor(pos.up());
        check_neighbor(pos.left());
        check_neighbor(pos.right());
        check_neighbor(pos.down());

        neighboring_groups
    }

    pub fn group_at(&self, pos: Pos<BS>) -> GroupId<BS> {
        self.pos_to_group[pos.index()]
    }

    fn _liberties_and_owners_of_groups(
        board: &Board<BS>,
        pos_to_group: &GroupedStones<BS>,
    ) -> Vec<GroupInfo<BS>> {
        let mut liberties_and_owners = vec![
            GroupInfo::Unknown {
                liberties: NumStones::ZERO
            };
            pos_to_group.num_groups().into_usize()
        ];

        for pos in Pos::all_positions() {
            if let Some(owner) = board[pos] {
                // It's a filled cell. Remember the owner of this group
                let group = pos_to_group.group_at(pos).into_usize();
                match liberties_and_owners[group] {
                    GroupInfo::EmptyStonesGroup => {
                        liberties_and_owners[group] = GroupInfo::PlayerGroup {
                            owner,
                            liberties: NumStones::ZERO,
                        };
                    }
                    GroupInfo::Unknown { liberties } => {
                        liberties_and_owners[group] = GroupInfo::PlayerGroup { owner, liberties };
                    }
                    GroupInfo::PlayerGroup {
                        owner: actual_owner,
                        liberties: _liberties,
                    } => {
                        assert_eq!(owner, actual_owner);
                    }
                }
            } else {
                let group = pos_to_group.group_at(pos).into_usize();
                liberties_and_owners[group] = GroupInfo::EmptyStonesGroup;

                // It's an empty cell. Any neighboring group that is occupied will get a liberty added.
                // But we need to make sure we only add it once if two neighboring fields are from the same group.
                // This code also adds liberties to the group representing the empty cells but that doesn't really matter.
                let mut groups_to_add_liberty_to: SmallSet<[GroupId<BS>; 5]> = SmallSet::new();
                groups_to_add_liberty_to.insert(pos_to_group.group_at(pos));
                if let Some(left) = pos.left() {
                    groups_to_add_liberty_to.insert(pos_to_group.group_at(left));
                }
                if let Some(top) = pos.up() {
                    groups_to_add_liberty_to.insert(pos_to_group.group_at(top));
                }
                if let Some(right) = pos.right() {
                    groups_to_add_liberty_to.insert(pos_to_group.group_at(right));
                }
                if let Some(down) = pos.down() {
                    groups_to_add_liberty_to.insert(pos_to_group.group_at(down));
                }
                for group_index in groups_to_add_liberty_to.iter() {
                    match &mut liberties_and_owners[group_index.into_usize()] {
                        GroupInfo::Unknown { liberties } => *liberties += NumStones::ONE,
                        GroupInfo::PlayerGroup {
                            owner: _owner,
                            liberties,
                        } => *liberties += NumStones::ONE,
                        GroupInfo::EmptyStonesGroup => {
                            // ignore, we don't care about the number of liberties of empty groups
                        }
                    }
                }
            }
        }

        debug_assert!(
            !liberties_and_owners
                .iter()
                .any(|info| matches!(info, GroupInfo::Unknown { .. })),
            "Analysis left an Unknown group behind"
        );

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
