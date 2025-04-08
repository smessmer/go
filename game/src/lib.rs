#![feature(generic_const_exprs)]

mod analysis;
mod board;
mod game;
mod gamelog;
mod group_stones;
mod sgf_parser;
mod utils;

pub use board::{
    Board, BoardSize, BoardSize9x9, BoardSize13x13, BoardSize19x19, NumStones, PlaceStoneError,
    Player, Pos,
};
pub use game::Game;
pub use sgf_parser::{Move, Outcome, OutcomeMargin, SgfGame, parse_sgf};

#[cfg(test)]
mod testutils;
