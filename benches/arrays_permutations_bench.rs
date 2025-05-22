use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use RustyLeetCode::arrays::medium::permutations::{
    in_place_swapping, iteratively_heap, lazy_yield_iterator::Permutations, recursively_simple_eager,
    recursively_with_backtracking,
};

fn get_sample_input() -> Vec<i32> {
    vec![1, 2, 3]
}

fn bench_permute(c: &mut Criterion) {
    let input = get_sample_input();

    c.bench_function("Permute - [1,2,3] - Simple Eager Recursion", |b| {
        b.iter(|| {
            let _ = recursively_simple_eager::permute(black_box(input.clone()));
        })
    });

    c.bench_function("Permute - [1,2,3] - Recursion with Backtracking", |b| {
        b.iter(|| {
            let _ = recursively_with_backtracking::permute(black_box(input.clone()));
        })
    });

    c.bench_function("Permute - [1,2,3] - In-Place Swapping", |b| {
        b.iter(|| {
            let _ = in_place_swapping::permute(black_box(input.clone()));
        })
    });

    c.bench_function("Permute - [1,2,3] - Iterative Heap", |b| {
        b.iter(|| {
            let _ = iteratively_heap::permute(black_box(input.clone()));
        })
    });

    c.bench_function("Permute - [1,2,3] - Lazy Yield Iterator", |b| {
        b.iter(|| {
            let iter = Permutations::new(black_box(input.clone()));
            let _: Vec<_> = iter.collect();
        })
    });
}

criterion_group!(benches, bench_permute);
criterion_main!(benches);
