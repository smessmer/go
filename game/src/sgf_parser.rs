use anyhow::{Context, Result, anyhow, bail, ensure};
use sgf_parse::go::Prop;

use crate::{BoardSize19x19, Game, Player, Pos};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SgfGame {
    // TODO In our integration tests, test that we're getting to the same outcome if the outcome is by points.
    pub outcome: Outcome,
    pub moves: Vec<Move>,
}

impl SgfGame {
    pub fn game_position_after_num_moves(&self, move_index: usize) -> Result<Game<BoardSize19x19>> {
        let mut game = Game::new();
        let mut moves = self.moves.iter();
        for i in 0..move_index {
            match moves.next() {
                None => panic!("Expected {move_index} moves but only saw {i}"),
                Some(Move::Pass) => {
                    game.pass_turn();
                }
                Some(Move::Place { x, y }) => {
                    game.place_stone(Pos::from_xy(usize::from(*x), usize::from(*y)))
                        .unwrap();
                }
            }
        }
        Ok(game)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    WithWinner {
        winner: Player,
        margin: OutcomeMargin,
    },
    Draw,
    Void,
    Unfinished,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeMargin {
    ByResign,
    ByTime,
    ByForfeit,
    ByPoints {
        // Multiplied by two so we can represent half points
        points_times_two: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Pass,
    Place { x: u8, y: u8 },
}

fn parse_outcome(input: &str) -> Result<Outcome> {
    if let Some(margin) = input.strip_prefix("W+") {
        let margin = parse_margin(margin)?;
        Ok(Outcome::WithWinner {
            winner: Player::White,
            margin,
        })
    } else if let Some(margin) = input.strip_prefix("B+") {
        let margin = parse_margin(margin)?;
        Ok(Outcome::WithWinner {
            winner: Player::White,
            margin,
        })
    } else if input == "Jigo" {
        Ok(Outcome::Draw)
    } else if input == "Void" {
        Ok(Outcome::Void)
    } else if input == "Unfinished" {
        Ok(Outcome::Unfinished)
    } else if input == "Unknown" {
        Ok(Outcome::Unknown)
    } else {
        Err(anyhow!("Unknown outcome: {}", input))
    }
}

fn parse_margin(input: &str) -> Result<OutcomeMargin> {
    if input == "R" {
        Ok(OutcomeMargin::ByResign)
    } else if input == "T" {
        Ok(OutcomeMargin::ByTime)
    } else if input == "F" {
        Ok(OutcomeMargin::ByForfeit)
    } else if let Ok(points) = input.parse::<f32>() {
        let points_times_two = (points * 2.0) as u32;
        ensure!(
            (points_times_two as f32) - points * 2.0 < 0.0001,
            "Invalid points value: {}",
            input
        );
        Ok(OutcomeMargin::ByPoints { points_times_two })
    } else {
        Err(anyhow!("Unknown outcome margin: {}", input))
    }
}

pub fn parse_sgf(sgf: &str) -> Result<SgfGame> {
    let games = sgf_parse::go::parse(sgf)?.into_iter();
    let game = single(games).context("Expected exactly one game in the SGF file")?;
    let board_size = match game.get_property("SZ") {
        Some(Prop::SZ(size)) => size.clone(),
        None => (19, 19),
        _ => unreachable!(),
    };
    ensure!(
        board_size == (19, 19),
        "Expected board size to be 19x19 but was {board_size:?}"
    );
    let outcome = match game.get_property("RE") {
        Some(Prop::RE(outcome)) => parse_outcome(&outcome.text)?,
        _ => unreachable!(),
    };
    let mut current_player = Player::Black;

    let mut moves = Vec::new();
    let mut current_node = single(game.children())?;
    loop {
        if let Some(Prop::W(move_)) = current_node.get_property("W") {
            ensure!(
                current_node.get_property("B").is_none(),
                "Node has both a W and B property"
            );
            ensure!(current_player == Player::White, "Expected White's turn");
            current_player = Player::Black;
            let mov = parse_move(&move_);
            moves.push(mov);
        } else if let Some(Prop::B(move_)) = current_node.get_property("B") {
            ensure!(
                current_node.get_property("W").is_none(),
                "Node has both a W and B property"
            );
            ensure!(current_player == Player::Black, "Expected White's turn");
            current_player = Player::White;
            let mov = parse_move(&move_);
            moves.push(mov);
        } else {
            bail!("Node has neither a B nor a W property");
        }

        match current_node.children().next() {
            Some(next_node) => current_node = next_node,
            None => break,
        }
    }
    Ok(SgfGame { outcome, moves })
}

fn parse_move(input: &sgf_parse::go::Move) -> Move {
    match input {
        sgf_parse::go::Move::Pass => Move::Pass,
        sgf_parse::go::Move::Move(point) => Move::Place {
            x: point.x as u8,
            y: point.y as u8,
        },
    }
}

fn single<I>(mut iter: impl Iterator<Item = I>) -> Result<I> {
    let result = iter.next().ok_or_else(|| anyhow!("No element found"))?;
    if iter.next().is_none() {
        Ok(result)
    } else {
        Err(anyhow!("More than one element found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAME_SGF: &str = include_str!("../tests/sixteen_soldiers.sgf");

    #[test]
    fn test_parse_sgf() {
        let parsed = parse_sgf(GAME_SGF).unwrap();

        assert_eq!(
            parsed,
            SgfGame {
                outcome: Outcome::WithWinner {
                    winner: Player::White,
                    margin: OutcomeMargin::ByResign,
                },
                moves: vec![
                    Move::Place { x: 16, y: 2 },
                    Move::Place { x: 3, y: 15 },
                    Move::Place { x: 16, y: 16 },
                    Move::Place { x: 3, y: 3 },
                    Move::Place { x: 9, y: 9 },
                    Move::Place { x: 15, y: 9 },
                    Move::Place { x: 2, y: 9 },
                    Move::Place { x: 5, y: 9 },
                    Move::Place { x: 9, y: 2 },
                    Move::Place { x: 9, y: 5 },
                    Move::Place { x: 9, y: 16 },
                    Move::Place { x: 9, y: 13 },
                    Move::Place { x: 11, y: 9 },
                    Move::Place { x: 15, y: 3 },
                    Move::Place { x: 15, y: 2 },
                    Move::Place { x: 14, y: 3 },
                    Move::Place { x: 17, y: 4 },
                    Move::Place { x: 15, y: 6 },
                    Move::Place { x: 16, y: 14 },
                    Move::Place { x: 14, y: 13 },
                    Move::Place { x: 9, y: 11 },
                    Move::Place { x: 13, y: 15 },
                    Move::Place { x: 6, y: 16 },
                    Move::Place { x: 6, y: 13 },
                    Move::Place { x: 2, y: 6 },
                    Move::Place { x: 6, y: 3 },
                    Move::Place { x: 6, y: 1 },
                    Move::Place { x: 2, y: 11 },
                    Move::Place { x: 6, y: 2 },
                    Move::Place { x: 2, y: 4 },
                    Move::Place { x: 3, y: 8 },
                    Move::Place { x: 11, y: 11 },
                    Move::Place { x: 11, y: 10 },
                    Move::Place { x: 10, y: 11 },
                    Move::Place { x: 10, y: 10 },
                    Move::Place { x: 8, y: 10 },
                    Move::Place { x: 1, y: 11 },
                    Move::Place { x: 1, y: 12 },
                    Move::Place { x: 1, y: 10 },
                    Move::Place { x: 4, y: 16 },
                    Move::Place { x: 11, y: 16 },
                    Move::Place { x: 10, y: 15 },
                    Move::Place { x: 9, y: 15 },
                    Move::Place { x: 9, y: 10 },
                    Move::Place { x: 10, y: 8 },
                    Move::Place { x: 8, y: 17 },
                    Move::Place { x: 7, y: 16 },
                    Move::Place { x: 10, y: 16 },
                    Move::Place { x: 10, y: 17 },
                    Move::Place { x: 9, y: 17 },
                    Move::Place { x: 11, y: 17 },
                    Move::Place { x: 11, y: 15 },
                    Move::Place { x: 7, y: 17 },
                    Move::Place { x: 12, y: 16 },
                    Move::Place { x: 12, y: 17 },
                    Move::Place { x: 13, y: 17 },
                    Move::Place { x: 14, y: 16 },
                    Move::Place { x: 13, y: 16 },
                    Move::Place { x: 8, y: 16 },
                    Move::Place { x: 12, y: 18 },
                    Move::Place { x: 8, y: 18 },
                    Move::Place { x: 5, y: 17 },
                    Move::Place { x: 7, y: 14 },
                    Move::Place { x: 3, y: 12 },
                    Move::Place { x: 6, y: 14 },
                    Move::Place { x: 11, y: 7 },
                    Move::Place { x: 7, y: 8 },
                    Move::Place { x: 5, y: 7 },
                    Move::Place { x: 9, y: 6 },
                    Move::Place { x: 1, y: 5 },
                    Move::Place { x: 1, y: 8 },
                    Move::Place { x: 10, y: 6 },
                    Move::Place { x: 8, y: 6 },
                    Move::Place { x: 16, y: 13 },
                    Move::Place { x: 17, y: 13 },
                    Move::Place { x: 17, y: 12 },
                    Move::Place { x: 17, y: 14 },
                    Move::Place { x: 10, y: 2 },
                    Move::Place { x: 9, y: 3 },
                    Move::Place { x: 10, y: 3 },
                    Move::Place { x: 3, y: 1 },
                    Move::Place { x: 4, y: 1 },
                    Move::Place { x: 4, y: 2 },
                    Move::Place { x: 3, y: 2 },
                    Move::Place { x: 5, y: 1 },
                    Move::Place { x: 2, y: 1 },
                    Move::Place { x: 4, y: 0 },
                    Move::Place { x: 14, y: 2 },
                    Move::Place { x: 17, y: 6 },
                    Move::Place { x: 16, y: 12 },
                    Move::Place { x: 2, y: 13 },
                    Move::Place { x: 1, y: 13 },
                    Move::Place { x: 2, y: 14 },
                    Move::Place { x: 2, y: 12 },
                    Move::Place { x: 4, y: 14 },
                    Move::Place { x: 3, y: 14 },
                    Move::Place { x: 5, y: 13 },
                    Move::Place { x: 3, y: 13 },
                    Move::Place { x: 8, y: 11 },
                    Move::Place { x: 7, y: 10 },
                    Move::Place { x: 7, y: 11 },
                    Move::Place { x: 6, y: 10 },
                    Move::Place { x: 12, y: 11 },
                    Move::Place { x: 12, y: 12 },
                    Move::Place { x: 13, y: 10 },
                    Move::Place { x: 16, y: 18 },
                    Move::Place { x: 17, y: 18 },
                    Move::Place { x: 6, y: 11 },
                    Move::Place { x: 8, y: 13 },
                    Move::Place { x: 10, y: 12 },
                    Move::Place { x: 13, y: 12 },
                    Move::Place { x: 6, y: 12 },
                    Move::Place { x: 7, y: 13 },
                    Move::Place { x: 13, y: 13 },
                    Move::Place { x: 10, y: 1 },
                    Move::Place { x: 11, y: 1 },
                    Move::Place { x: 10, y: 4 },
                    Move::Place { x: 11, y: 4 },
                    Move::Place { x: 9, y: 1 },
                    Move::Place { x: 9, y: 4 },
                    Move::Place { x: 1, y: 6 },
                    Move::Place { x: 2, y: 0 },
                    Move::Place { x: 12, y: 13 },
                    Move::Place { x: 11, y: 12 },
                    Move::Place { x: 10, y: 14 },
                    Move::Place { x: 11, y: 14 },
                    Move::Place { x: 9, y: 14 },
                    Move::Place { x: 14, y: 12 },
                    Move::Place { x: 13, y: 7 },
                    Move::Place { x: 13, y: 11 },
                    Move::Place { x: 16, y: 8 },
                    Move::Place { x: 15, y: 1 },
                    Move::Place { x: 16, y: 1 },
                    Move::Place { x: 13, y: 1 },
                    Move::Place { x: 16, y: 9 },
                    Move::Place { x: 16, y: 3 },
                    Move::Place { x: 17, y: 3 },
                    Move::Place { x: 13, y: 6 },
                    Move::Place { x: 15, y: 0 },
                    Move::Place { x: 17, y: 10 },
                    Move::Place { x: 14, y: 7 },
                    Move::Place { x: 14, y: 6 },
                    Move::Place { x: 11, y: 0 },
                    Move::Place { x: 12, y: 1 },
                    Move::Place { x: 8, y: 4 },
                    Move::Place { x: 5, y: 4 },
                    Move::Place { x: 1, y: 2 },
                    Move::Place { x: 2, y: 2 },
                    Move::Place { x: 4, y: 4 },
                    Move::Place { x: 4, y: 3 },
                    Move::Place { x: 5, y: 3 },
                    Move::Place { x: 5, y: 2 },
                    Move::Place { x: 8, y: 5 },
                    Move::Place { x: 10, y: 5 },
                    Move::Place { x: 5, y: 3 },
                    Move::Place { x: 15, y: 15 }
                ]
            }
        )
    }
}
