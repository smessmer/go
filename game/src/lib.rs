#![feature(generic_const_exprs)]

mod board;
mod game;
mod group_stones;

pub use board::{Board, PlaceStoneError, Player};
pub use game::Game;

#[cfg(test)]
mod testutils;
