use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use RustyLeetCode::dynamic_programming::easy::tribanacci_number::{
    bottom_up_dp, matrix_exponentiation, space_optimized_dp, top_down_memoization,
};

fn bench_tribonacci_med_size_input(c: &mut Criterion) {
    let m = medium_input();

    c.bench_function("top_down - medium", |b| {
        b.iter(|| top_down_memoization::tribonacci(black_box(m)))
    });

    c.bench_function("bottom_up - medium", |b| {
        b.iter(|| bottom_up_dp::tribonacci(black_box(m)))
    });

    c.bench_function("space_optimized - medium", |b| {
        b.iter(|| space_optimized_dp::tribonacci(black_box(m)))
    });

    c.bench_function("matrix_expo - medium", |b| {
        b.iter(|| matrix_exponentiation::tribonacci(black_box(m)))
    });
}

fn bench_tribonacci_large_size_input(c: &mut Criterion) {
    let l = large_input();

    c.bench_function("top_down - large", |b| {
        b.iter(|| top_down_memoization::tribonacci(black_box(l)))
    });

    c.bench_function("bottom_up - large", |b| {
        b.iter(|| bottom_up_dp::tribonacci(black_box(l)))
    });

    c.bench_function("space_optimized - large", |b| {
        b.iter(|| space_optimized_dp::tribonacci(black_box(l)))
    });

    c.bench_function("matrix_expo - large", |b| {
        b.iter(|| matrix_exponentiation::tribonacci(black_box(l)))
    });
}

fn medium_input() -> i32 {
    25
}

fn large_input() -> i32 {
    37 // Leetcode constraint max
}

criterion_group!(
    benches,
    bench_tribonacci_med_size_input,
    bench_tribonacci_large_size_input
);
criterion_main!(benches);
