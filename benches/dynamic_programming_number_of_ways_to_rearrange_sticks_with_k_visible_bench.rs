use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use RustyLeetCode::dynamic_programming::hard::number_of_ways_to_rearrange_sticks_with_k_visible::{
    bottom_up_dp, matrix_exponentiation, top_down_memoization,
};

fn medium_input() -> (i32, i32) {
    (20, 5)
}

fn large_input() -> (i32, i32) {
    (300, 25)
}

fn benchmark_rearrange_sticks_med_input_size(c: &mut Criterion) {
    let (mn, mk) = medium_input();

    c.bench_function("bottom_up - medium", |b| {
        b.iter(|| bottom_up_dp::rearrange_sticks(black_box(mn), black_box(mk)))
    });

    c.bench_function("top_down - medium", |b| {
        b.iter(|| top_down_memoization::rearrange_sticks(black_box(mn), black_box(mk)))
    });

    c.bench_function("matrix_expo - medium", |b| {
        b.iter(|| matrix_exponentiation::rearrange_sticks(black_box(mn), black_box(mk)))
    });
}

fn benchmark_rearrange_sticks_large_input_size(c: &mut Criterion) {
    let (ln, lk) = large_input();

    c.bench_function("bottom_up - large", |b| {
        b.iter(|| bottom_up_dp::rearrange_sticks(black_box(ln), black_box(lk)))
    });

    c.bench_function("top_down - large", |b| {
        b.iter(|| top_down_memoization::rearrange_sticks(black_box(ln), black_box(lk)))
    });

    c.bench_function("matrix_expo - large", |b| {
        b.iter(|| matrix_exponentiation::rearrange_sticks(black_box(ln), black_box(lk)))
    });
}

criterion_group!(
    benches,
    benchmark_rearrange_sticks_med_input_size,
    benchmark_rearrange_sticks_large_input_size
);
criterion_main!(benches);
