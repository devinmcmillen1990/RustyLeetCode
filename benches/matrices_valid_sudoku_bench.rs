use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use RustyLeetCode::matrices::medium::valid_sudoku::{bitmasking_instead_of_hashsets, iteratively};

fn get_sample_board() -> Vec<Vec<char>> {
    vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ]
}

pub fn bench_hashset(c: &mut Criterion) {
    let board = get_sample_board();
    c.bench_function("is_valid_sudoku_hashset", |b| {
        b.iter(|| {
            let _ = iteratively::is_valid_sudoku(black_box(board.clone()));
        })
    });
}

pub fn bench_bitmask(c: &mut Criterion) {
    let board = get_sample_board();
    c.bench_function("is_valid_sudoku_bitmask", |b| {
        b.iter(|| {
            let _ = bitmasking_instead_of_hashsets::is_valid_sudoku(black_box(board.clone()));
        })
    });
}

criterion_group!(sudoku_benches, bench_hashset, bench_bitmask);
criterion_main!(sudoku_benches);
