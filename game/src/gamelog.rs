use crate::{Board, BoardSize};

pub struct GameLog<BS: BoardSize>
where
    [(); bitvec::mem::elts::<usize>(2 * <BS as BoardSize>::SIZE * <BS as BoardSize>::SIZE)]:,
{
    komi: f32,
    initial_board: Board<BS>,
}
