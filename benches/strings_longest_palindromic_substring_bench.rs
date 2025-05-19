use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use RustyLeetCode::strings::medium::longest_palindromic_substring::{
    brute_force, center_expansion, dynamic_programming, manachers_algorithm,
};

/// Small input: Basic test with a single palindrome
const SMALL_INPUT: &str = "babad";

/// Medium input: Multiple potential palindromes
const MEDIUM_INPUT: &str = "abcbabccbabcbabccba";

/// Large input: Repeated characters to form large palindromes
fn generate_large_input(size: usize) -> String {
    "a".repeat(size / 2) + "racecar" + &"a".repeat(size / 2)
}

/// Benchmark for small inputs
fn bench_small(c: &mut Criterion) {
    let input = SMALL_INPUT.to_string();

    c.bench_function("Brute Force - Small", |b| {
        b.iter(|| {
            let _ = brute_force::longest_palindromic_substring(black_box(input.clone()));
        })
    });

    c.bench_function("Center Expand - Small", |b| {
        b.iter(|| {
            let _ = center_expansion::longest_palindromic_substring(black_box(input.clone()));
        })
    });

    c.bench_function("Dynamic Programming - Small", |b| {
        b.iter(|| {
            let _ = dynamic_programming::longest_palindromic_substring(black_box(input.clone()));
        })
    });

    c.bench_function("Manacher's Algorithm - Small", |b| {
        b.iter(|| {
            let _ = manachers_algorithm::longest_palindromic_substring(black_box(input.clone()));
        })
    });
}

/// Benchmark for medium inputs
fn bench_medium(c: &mut Criterion) {
    let input = MEDIUM_INPUT.to_string();

    // // NOTE: Skipping for Medium Size inputs because its too slow.
    // c.bench_function("Brute Force - Medium", |b| {
    //     b.iter(|| {
    //         let _ = brute_force::longest_palindromic_substring(black_box(input.clone()));
    //     })
    // });

    c.bench_function("Center Expand - Medium", |b| {
        b.iter(|| {
            let _ = center_expansion::longest_palindromic_substring(black_box(input.clone()));
        })
    });

    c.bench_function("Dynamic Programming - Medium", |b| {
        b.iter(|| {
            let _ = dynamic_programming::longest_palindromic_substring(black_box(input.clone()));
        })
    });

    c.bench_function("Manacher's Algorithm - Medium", |b| {
        b.iter(|| {
            let _ = manachers_algorithm::longest_palindromic_substring(black_box(input.clone()));
        })
    });
}

/// Benchmark for large inputs
fn bench_large(c: &mut Criterion) {
    let input = generate_large_input(10_000); // Adjust size for performance testing

    // // NOTE: Skipping for Large Size inputs because its too slow
    // c.bench_function("Brute Force - Large", |b| {
    //     b.iter(|| {
    //         let _ = brute_force::longest_palindromic_substring(black_box(input.clone()));
    //     })
    // });

    c.bench_function("Center Expand - Large", |b| {
        b.iter(|| {
            let _ = center_expansion::longest_palindromic_substring(black_box(input.clone()));
        })
    });

    // // NOTE: Skipping for Large Size inputs because its too slow
    // c.bench_function("Dynamic Programming - Large", |b| {
    //     b.iter(|| {
    //         let _ = dynamic_programming::longest_palindromic_substring(black_box(input.clone()));
    //     })
    // });

    c.bench_function("Manacher's Algorithm - Large", |b| {
        b.iter(|| {
            let _ = manachers_algorithm::longest_palindromic_substring(black_box(input.clone()));
        })
    });
}

criterion_group!(benches, bench_small, bench_medium, bench_large);
criterion_main!(benches);
