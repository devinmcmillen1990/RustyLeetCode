use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use RustyLeetCode::arrays::medium::permutations2::{
    iteratively_heap_with_hashset_dedup, lazy_yield_iterator::UniquePermutations,
    recursively_simple_eager_with_hashset, recursively_with_backtracking_with_hashset_dedup,
    recursively_with_backtracking_with_in_place_swaps_and_local_hashset,
    recursively_with_backtracking_with_vec_for_dedups,
};

fn get_sample_input() -> Vec<i32> {
    vec![1, 1, 2]
}

fn bench_permute_unique(c: &mut Criterion) {
    let input = get_sample_input();

    c.bench_function("Permute II - [1,1,2] - Simple Eager Recursion", |b| {
        b.iter(|| {
            let _ = recursively_simple_eager_with_hashset::permute_unique(black_box(input.clone()));
        })
    });

    c.bench_function("Permute II - [1,1,2] - Recursion with Backtracking", |b| {
        b.iter(|| {
            let _ = recursively_with_backtracking_with_vec_for_dedups::permute_unique(black_box(
                input.clone(),
            ));
        })
    });

    c.bench_function("Permute II - [1,1,2] - Heap + Dedup", |b| {
        b.iter(|| {
            let _ = recursively_with_backtracking_with_hashset_dedup::permute_unique(black_box(
                input.clone(),
            ));
        })
    });

    c.bench_function("Permute II - [1,1,2] - Iteratively Heap + Dedup", |b| {
        b.iter(|| {
            let _ = iteratively_heap_with_hashset_dedup::permute_unique(black_box(input.clone()));
        })
    });

    c.bench_function("Permute II - [1,1,2] - Iteratively In-Place Swap + Dedup", |b| {
        b.iter(|| {
            let _ = recursively_with_backtracking_with_in_place_swaps_and_local_hashset::permute_unique(black_box(
                input.clone(),
            ));
        })
    });

    c.bench_function("Permute II - [1,1,2] - Lazy Yield Iterator", |b| {
        b.iter(|| {
            let iter = UniquePermutations::new(black_box(input.clone()));
            let _: Vec<_> = iter.collect();
        })
    });
}

criterion_group!(benches, bench_permute_unique);
criterion_main!(benches);
