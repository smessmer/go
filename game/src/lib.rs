#![feature(generic_const_exprs)]

mod analysis;
mod board;
mod game;
mod group_stones;
mod utils;

pub use board::{
    Board, BoardSize, BoardSize9x9, BoardSize13x13, BoardSize19x19, NumStones, PlaceStoneError,
    Player, Pos,
};
pub use game::Game;

#[cfg(test)]
mod testutils;
