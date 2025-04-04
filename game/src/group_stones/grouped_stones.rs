use crate::BoardSize;
#[cfg(test)]
use derive_where::derive_where;

use super::group_id::GroupId;

#[cfg_attr(test, derive_where(PartialEq, Eq, Debug))]
pub struct GroupedStones<BS: BoardSize>
where
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    groups: [GroupId<BS>; <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE],

    num_groups: GroupId<BS>,
}

impl<BS: BoardSize> GroupedStones<BS>
where
    [(); <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE]:,
{
    pub fn new(
        groups: [GroupId<BS>; <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE],
        num_groups: GroupId<BS>,
    ) -> Self {
        Self { groups, num_groups }
    }

    pub fn group_at(&self, x: usize, y: usize) -> GroupId<BS> {
        assert!(
            x < <BS as BoardSize>::SIZE && y < <BS as BoardSize>::SIZE,
            "Coordinates out of bounds"
        );
        self.groups[y * <BS as BoardSize>::SIZE + x]
    }

    pub fn num_groups(&self) -> GroupId<BS> {
        self.num_groups
    }
}
