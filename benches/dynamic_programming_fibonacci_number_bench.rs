use criterion::{criterion_group, criterion_main, Criterion};
use RustyLeetCode::dynamic_programming::easy::fibonacci_number::{
    binets_formula_closed_form, bottom_up_dp, matrix_exponentiation, recursively_with_memoization,
    space_optimized_dp,
};

fn bench_fib_medium_n(c: &mut Criterion) {
    let medium_n = 30;

    c.bench_function("Fibonacci - Medium - Recursively with Memoization", |b| {
        b.iter(|| recursively_with_memoization::fib(medium_n))
    });
    c.bench_function("Fibonacci - Medium - Bottom Up DP", |b| {
        b.iter(|| bottom_up_dp::fib(medium_n))
    });
    c.bench_function("Fibonacci - Medium - Space Optimized DP", |b| {
        b.iter(|| space_optimized_dp::fib(medium_n))
    });
    c.bench_function("Fibonacci - Medium - Matrix Exponentiation", |b| {
        b.iter(|| matrix_exponentiation::fib(medium_n))
    });
    c.bench_function("Fibonacci - Medium - Binet's Formula (Closed Form)", |b| {
        b.iter(|| binets_formula_closed_form::fib(medium_n))
    });
}

fn bench_fib_large_n(c: &mut Criterion) {
    let large_n = 75;

    c.bench_function("Fibonacci - Large - Recursively with Memoization", |b| {
        b.iter(|| recursively_with_memoization::fib(large_n))
    });
    c.bench_function("Fibonacci - Large - Bottom Up DP", |b| {
        b.iter(|| bottom_up_dp::fib(large_n))
    });
    c.bench_function("Fibonacci - Large - Space Optimized DP", |b| {
        b.iter(|| space_optimized_dp::fib(large_n))
    });
    c.bench_function("Fibonacci - Large - Matrix Exponentiation", |b| {
        b.iter(|| matrix_exponentiation::fib(large_n))
    });
}

criterion_group!(benches, bench_fib_medium_n, bench_fib_large_n);
criterion_main!(benches);
