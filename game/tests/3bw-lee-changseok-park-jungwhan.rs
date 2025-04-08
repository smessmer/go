#![feature(generic_const_exprs)]

// From http://gokifu.com/ , 2025-04-03, Lee Changseok vs Park Jungwhan, B+10.5
// This is a game with a group capture of more than one stone

use common_macros::hash_map;
use go_game::{Board, parse_sgf};
use pretty_assertions::assert_eq;

const GAME_SGF: &str = include_str!("3bw-lee-changseok-park-jungwhan.sgf");

#[test]
fn game_3bw_gokifu() {
    let sgf_game = parse_sgf(GAME_SGF).unwrap();

    let expected_boards = hash_map! {
        // ○ ●
        254 => Board::from_str(r#"
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ ● ● ○ _ ○
            _ ○ ○ ● ● ○ ○ _ ● _ ● _ _ _ ● ○ _ ○ _
            ○ _ ○ ○ ● ● ○ ● ● ● ○ ○ ● _ ● ○ ○ _ _
            ○ ○ ● ● ● _ ● _ ● ○ ● ● ● _ ● _ ○ _ _
            ● ● _ ● _ ● _ ● ○ ○ ○ ○ _ _ ● _ ○ _ _
            _ _ ● _ ● ○ ● ● ○ ○ _ ○ ○ ○ ● ○ _ ○ _
            _ ○ _ ● ○ ○ ● ○ _ ● ○ ● ● ○ ● ● ○ ○ _
            _ ● ● ● ● ○ ○ ○ _ _ _ ○ ● ● _ ● ● ○ ●
            _ ● _ _ ○ _ ○ _ ○ ○ ○ ○ _ ● ○ ○ ● ● _
            _ ○ ● ● ● ○ _ _ _ ● ● ○ ● _ ● ○ ○ ○ _
            _ ○ ● _ _ ● ● ○ ○ ● _ ● ● ● ● ○ _ _ _
            _ ○ ○ ○ ● ● _ ● ○ ● ● ○ ○ ○ ● _ _ ○ _
            _ ○ ● ● ○ ● ● ○ _ ○ ● ● ● ○ ○ ○ ○ ● _
            _ _ _ _ ○ _ ○ ● ○ ○ _ ○ ● _ _ _ ○ ○ _
            _ ● ○ _ _ _ _ _ _ ○ _ ● _ _ ○ ○ ● ● ●
            _ _ ○ _ ○ ○ ○ ○ ○ _ _ ● ○ _ ○ ● ● ○ ●
            _ _ ○ ● ● ○ ● ○ ● ○ _ ● _ ● ● ○ ○ ○ ○
            _ ○ _ ○ ● ● ● ● ● ● _ _ _ _ ● ● ○ _ ●
            _ _ ○ _ _ _ _ _ _ _ _ _ _ _ ● _ ● ○ _
        "#).unwrap(),
        255 => Board::from_str(r#"
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ ● ● ○ _ ○
            _ ○ ○ ● ● ○ ○ _ ● _ ● _ _ _ ● ○ _ ○ _
            ○ _ ○ ○ ● ● ○ ● ● ● ○ ○ ● _ ● ○ ○ _ _
            ○ ○ ● ● ● _ ● _ ● ○ ● ● ● _ ● _ ○ _ _
            ● ● _ ● _ ● _ ● ○ ○ ○ ○ _ _ ● _ ○ _ _
            _ _ ● _ ● ○ ● ● ○ ○ _ ○ ○ ○ ● ○ _ ○ _
            _ ○ _ ● ○ ○ ● ○ _ ● ○ ● ● ○ ● ● ○ ○ _
            _ ● ● ● ● ○ ○ ○ _ _ _ ○ ● ● _ ● ● ○ ●
            _ ● _ _ ○ _ ○ _ ○ ○ ○ ○ _ ● ○ ○ ● ● _
            _ ○ ● ● ● ○ _ _ _ ● ● ○ ● _ ● ○ ○ ○ _
            _ ○ ● _ _ ● ● ○ ○ ● _ ● ● ● ● ○ _ _ _
            _ ○ ○ ○ ● ● _ ● ○ ● ● ○ ○ ○ ● _ _ ○ _
            _ ○ ● ● ○ ● ● ○ _ ○ ● ● ● ○ ○ ○ ○ ● _
            _ _ _ _ ○ _ ○ ● ○ ○ _ ○ ● _ _ _ ○ ○ ○
            _ ● ○ _ _ _ _ _ _ ○ _ ● _ _ ○ ○ _ _ _
            _ _ ○ _ ○ ○ ○ ○ ○ _ _ ● ○ _ ○ _ _ ○ _
            _ _ ○ ● ● ○ ● ○ ● ○ _ ● _ ● ● ○ ○ ○ ○
            _ ○ _ ○ ● ● ● ● ● ● _ _ _ _ ● ● ○ _ ●
            _ _ ○ _ _ _ _ _ _ _ _ _ _ _ ● _ ● ○ _
        "#).unwrap(),
    };

    for (move_index, expected_board) in expected_boards.iter() {
        assert_eq!(
            expected_board,
            sgf_game.game_after_move_index(*move_index).unwrap().board()
        );
    }
}
