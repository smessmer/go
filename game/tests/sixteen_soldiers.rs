#![feature(generic_const_exprs)]

// From https://senseis.xmp.net/?SixteenSoldiers

use common_macros::hash_map;
use go_game::{Board, parse_sgf};

const GAME_SGF: &str = include_str!("sixteen_soldiers.sgf");

#[test]
fn sixteen_soldiers() {
    let sgf_game = parse_sgf(GAME_SGF).unwrap();

    let expected_boards = hash_map! {
        10 => Board::from_str(r#"
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ ○ _ _ _ _ _ _ ○ _ _
        _ _ _ ● _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ ● _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ ○ _ _ ● _ _ _ ○ _ _ _ _ _ ● _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ ● _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ○ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        "#).unwrap(),
        20 => Board::from_str(r#"
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ ○ _ _ _ _ _ ○ ○ _ _
        _ _ _ ● _ _ _ _ _ _ _ _ _ _ ● ● _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ○ _
        _ _ _ _ _ _ _ _ _ ● _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ● _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ ○ _ _ ● _ _ _ ○ _ ○ _ _ _ ● _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ ● _ _ _ _ ● _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ○ _ _
        _ _ _ ● _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ ○ _ _ _ _ _ _ ○ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        "#).unwrap(),
        30 => Board::from_str(r#"
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ○ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ○ _ _ ○ _ _ _ _ _ ○ ○ _ _
        _ _ _ ● _ _ ● _ _ _ _ _ _ _ ● ● _ _ _
        _ _ ● _ _ _ _ _ _ _ _ _ _ _ _ _ _ ○ _
        _ _ _ _ _ _ _ _ _ ● _ _ _ _ _ _ _ _ _
        _ _ ○ _ _ _ _ _ _ _ _ _ _ _ _ ● _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ ○ _ _ ● _ _ _ ○ _ ○ _ _ _ ● _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ ● _ _ _ _ _ _ ○ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ● _ _ ● _ _ _ _ ● _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ○ _ _
        _ _ _ ● _ _ _ _ _ _ _ _ _ ● _ _ _ _ _
        _ _ _ _ _ _ ○ _ _ ○ _ _ _ _ _ _ ○ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        "#).unwrap(),
        40 => Board::from_str(r#"
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ○ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ○ _ _ ○ _ _ _ _ _ ○ ○ _ _
        _ _ _ ● _ _ ● _ _ _ _ _ _ _ ● ● _ _ _
        _ _ ● _ _ _ _ _ _ _ _ _ _ _ _ _ _ ○ _
        _ _ _ _ _ _ _ _ _ ● _ _ _ _ _ _ _ _ _
        _ _ ○ _ _ _ _ _ _ _ _ _ _ _ _ ● _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ ○ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ ○ _ _ ● _ _ _ ○ _ ○ _ _ _ ● _ _ _
        _ ○ _ _ _ _ _ _ ● _ ○ ○ _ _ _ _ _ _ _
        _ ○ ● _ _ _ _ _ _ ○ ● ● _ _ _ _ _ _ _
        _ ● _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ● _ _ ● _ _ _ _ ● _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ○ _ _
        _ _ _ ● _ _ _ _ _ _ _ _ _ ● _ _ _ _ _
        _ _ _ _ ● _ ○ _ _ ○ _ _ _ _ _ _ ○ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        "#).unwrap(),
        50 => Board::from_str(r#"
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ○ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ○ _ _ ○ _ _ _ _ _ ○ ○ _ _
        _ _ _ ● _ _ ● _ _ _ _ _ _ _ ● ● _ _ _
        _ _ ● _ _ _ _ _ _ _ _ _ _ _ _ _ _ ○ _
        _ _ _ _ _ _ _ _ _ ● _ _ _ _ _ _ _ _ _
        _ _ ○ _ _ _ _ _ _ _ _ _ _ _ _ ● _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ ○ _ _ _ _ _ _ ○ _ _ _ _ _ _ _ _
        _ _ ○ _ _ ● _ _ _ ○ _ ○ _ _ _ ● _ _ _
        _ ○ _ _ _ _ _ _ ● ● ○ ○ _ _ _ _ _ _ _
        _ ○ ● _ _ _ _ _ _ ○ ● ● _ _ _ _ _ _ _
        _ ● _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ● _ _ ● _ _ _ _ ● _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ○ _ _
        _ _ _ ● _ _ _ _ _ ○ ● _ _ ● _ _ _ _ _
        _ _ _ _ ● _ ○ ○ _ ○ ● ○ _ _ _ _ ○ _ _
        _ _ _ _ _ _ _ _ ● ● ○ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        "#).unwrap(),
        60 => Board::from_str(r#"
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ○ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ○ _ _ ○ _ _ _ _ _ ○ ○ _ _
        _ _ _ ● _ _ ● _ _ _ _ _ _ _ ● ● _ _ _
        _ _ ● _ _ _ _ _ _ _ _ _ _ _ _ _ _ ○ _
        _ _ _ _ _ _ _ _ _ ● _ _ _ _ _ _ _ _ _
        _ _ ○ _ _ _ _ _ _ _ _ _ _ _ _ ● _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ ○ _ _ _ _ _ _ ○ _ _ _ _ _ _ _ _
        _ _ ○ _ _ ● _ _ _ ○ _ ○ _ _ _ ● _ _ _
        _ ○ _ _ _ _ _ _ ● ● ○ ○ _ _ _ _ _ _ _
        _ ○ ● _ _ _ _ _ _ ○ ● ● _ _ _ _ _ _ _
        _ ● _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ ● _ _ ● _ _ _ _ ● _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ○ _ _
        _ _ _ ● _ _ _ _ _ ○ ● ● _ ● _ _ _ _ _
        _ _ _ _ ● _ ○ ○ ○ ○ ● ○ ● ● ○ _ ○ _ _
        _ _ _ _ _ _ _ ○ ● ● ○ ○ ○ ● _ _ _ _ _
        _ _ _ _ _ _ _ _ _ _ _ _ ● _ _ _ _ _ _
        "#).unwrap(),
    };

    for (move_index, expected_board) in expected_boards.iter() {
        assert_eq!(
            expected_board,
            sgf_game
                .game_position_after_num_moves(*move_index)
                .unwrap()
                .board()
        );
    }
}
