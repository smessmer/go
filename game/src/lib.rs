#![feature(generic_const_exprs)]

mod board;
mod game;

pub use board::{Board, PlaceStoneError, Player};
pub use game::Game;
