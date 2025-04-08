// From https://senseis.xmp.net/?SixteenSoldiers

use anyhow::{Context, Result, anyhow, bail, ensure};
use go_game::Player;
use itertools::Itertools;
use sgf_parse::{SgfNode, SgfParseError, go::Prop};

use go_game::parse_sgf;

const GAME_SGF: &str = include_str!("sixteen_soldiers.sgf");

#[test]
fn sixteen_soldiers() {
    let game = parse_sgf(GAME_SGF).unwrap();
}
