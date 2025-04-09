use crate::{BoardSize, board::Pos};
use derive_where::derive_where;

use super::group_id::GroupId;

#[derive_where(PartialEq, Eq, Debug)]
pub struct GroupedStones<BS: BoardSize>
where
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    pos_to_group: [GroupId<BS>; <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE],

    num_groups: GroupId<BS>,
}

impl<BS: BoardSize> GroupedStones<BS>
where
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    pub fn new(
        pos_to_group: [GroupId<BS>; <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE],
        num_groups: GroupId<BS>,
    ) -> Self {
        Self {
            pos_to_group,
            num_groups,
        }
    }

    pub fn group_at(&self, pos: Pos<BS>) -> GroupId<BS> {
        self.pos_to_group[pos.index()]
    }

    pub fn num_groups(&self) -> GroupId<BS> {
        self.num_groups
    }

    pub fn into(self) -> [GroupId<BS>; <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE] {
        self.pos_to_group
    }
}
