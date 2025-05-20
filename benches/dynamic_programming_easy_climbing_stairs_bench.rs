use criterion::{criterion_group, criterion_main, Criterion};
use RustyLeetCode::dynamic_programming::easy::climbing_stairs::{
    bottom_up_dp_array, optimized_dp_two_vars, recursively_with_memoization,
};

pub fn bench_medium_size_dataset(c: &mut Criterion) {
    let medium_n = 1000;

    c.bench_function(
        "Climbing Stairs - LaMediumrge - Recursive with Memoization",
        |b| b.iter(|| recursively_with_memoization::climb_stairs(medium_n)),
    );

    c.bench_function("Climbing Stairs - Medium - Bottom Up DP Array", |b| {
        b.iter(|| bottom_up_dp_array::climb_stairs(medium_n))
    });

    c.bench_function("Climbing Stairs - Medium - Optimized DP (2 Vars)", |b| {
        b.iter(|| optimized_dp_two_vars::climb_stairs(medium_n))
    });
}

pub fn bench_large_size_dataset(c: &mut Criterion) {
    let large_n = 10_000;

    c.bench_function(
        "Climbing Stairs - Large - Recursive with Memoization",
        |b| b.iter(|| recursively_with_memoization::climb_stairs(large_n)),
    );

    c.bench_function("Climbing Stairs - Large - Bottom Up DP Array", |b| {
        b.iter(|| bottom_up_dp_array::climb_stairs(large_n))
    });

    c.bench_function("Climbing Stairs - Large - Optimized DP (2 Vars)", |b| {
        b.iter(|| optimized_dp_two_vars::climb_stairs(large_n))
    });
}

criterion_group!(benches, bench_medium_size_dataset, bench_large_size_dataset);
criterion_main!(benches);
