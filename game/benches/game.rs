#![feature(generic_const_exprs)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use go_game::{BoardSize19x19, Game, Move, Pos, SgfGame};

const GAME1_SGF: &str = include_str!("../tests/sixteen_soldiers.sgf");
const GAME2_SGF: &str = include_str!("../tests/3bw-lee-changseok-park-jungwhan.sgf");
const GAME3_SGF: &str = include_str!("../tests/3bw-gokifu-han-chongjin-le-changho.sgf");

fn simulate_game(sgf_game: &SgfGame) {
    let mut game = Game::<BoardSize19x19>::new();
    for game_move in &sgf_game.moves {
        match game_move {
            Move::Pass => {
                game.pass_turn();
            }
            Move::Place { x, y } => {
                game.place_stone(Pos::from_xy(usize::from(*x), usize::from(*y)))
                    .unwrap();
            }
        }
    }
    // TODO Access outcome
    black_box(game);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let game1 = go_game::parse_sgf(GAME1_SGF).unwrap();
    let game2 = go_game::parse_sgf(GAME2_SGF).unwrap();
    let game3 = go_game::parse_sgf(GAME3_SGF).unwrap();

    c.bench_function("game1", |b| b.iter(|| simulate_game(&game1)));
    c.bench_function("game2", |b| b.iter(|| simulate_game(&game2)));
    c.bench_function("game3", |b| b.iter(|| simulate_game(&game3)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
