use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use RustyLeetCode::strings::medium::longest_substring_without_repeating_chars::{
    brute_force, sliding_window, sliding_window_ascii, sliding_window_linked_hashmap,
};

/// Benchmark input sizes
const SMALL_INPUT: &str = "abcabcbb";
const MEDIUM_INPUT: &str = "abcdefghijklmnopqrstuvwxyz";

/// Benchmark for small inputs
fn bench_small(c: &mut Criterion) {
    c.bench_function("Brute Force - Small", |b| {
        b.iter(|| {
            let _ = brute_force::length_of_longest_substring(black_box(SMALL_INPUT.to_string()));
        })
    });

    c.bench_function("Sliding Window - Small", |b| {
        b.iter(|| {
            let _ = sliding_window::length_of_longest_substring(black_box(SMALL_INPUT.to_string()));
        })
    });

    c.bench_function("ASCII Sliding Window - Small", |b| {
        b.iter(|| {
            let _ = sliding_window_ascii::length_of_longest_substring(black_box(
                SMALL_INPUT.to_string(),
            ));
        })
    });

    c.bench_function("Linked HashMap Sliding Window - Small", |b| {
        b.iter(|| {
            let _ = sliding_window_linked_hashmap::length_of_longest_substring(black_box(
                SMALL_INPUT.to_string(),
            ));
        })
    });
}

/// Benchmark for medium inputs
fn bench_medium(c: &mut Criterion) {
    c.bench_function("Brute Force - Medium", |b| {
        b.iter(|| {
            let _ = brute_force::length_of_longest_substring(black_box(MEDIUM_INPUT.to_string()));
        })
    });

    c.bench_function("Sliding Window - Medium", |b| {
        b.iter(|| {
            let _ =
                sliding_window::length_of_longest_substring(black_box(MEDIUM_INPUT.to_string()));
        })
    });

    c.bench_function("ASCII Sliding Window - Medium", |b| {
        b.iter(|| {
            let _ = sliding_window_ascii::length_of_longest_substring(black_box(
                MEDIUM_INPUT.to_string(),
            ));
        })
    });

    c.bench_function("Linked HashMap - Medium", |b| {
        b.iter(|| {
            let _ = sliding_window_linked_hashmap::length_of_longest_substring(black_box(
                MEDIUM_INPUT.to_string(),
            ));
        })
    });
}

/// Benchmark for large inputs
fn bench_large(c: &mut Criterion) {
    let large_input = "a".repeat(100_000);

    c.bench_function("Sliding Window - Large", |b| {
        b.iter(|| {
            let _ = sliding_window::length_of_longest_substring(black_box(large_input.clone()));
        })
    });

    c.bench_function("ASCII Sliding Window - Large", |b| {
        b.iter(|| {
            let _ =
                sliding_window_ascii::length_of_longest_substring(black_box(large_input.clone()));
        })
    });

    c.bench_function("Linked HashMap Sliding Window - Large", |b| {
        b.iter(|| {
            let _ = sliding_window_linked_hashmap::length_of_longest_substring(black_box(
                large_input.clone(),
            ));
        })
    });
}

criterion_group!(benches, bench_small, bench_medium, bench_large);
criterion_main!(benches);
