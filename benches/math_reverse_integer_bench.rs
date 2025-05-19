use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use RustyLeetCode::math::medium::reverse_integer::{
    base_iterative, base_recursive, divide_and_conquer, string_conversion,
};

/// Generate a large input value for benchmarking
fn generate_large_input(size: usize) -> i32 {
    let mut num_str = "1".to_string();
    for _ in 0..size {
        num_str.push('0');
    }
    num_str.parse::<i32>().unwrap_or(i32::MAX)
}

fn bench_medium(c: &mut Criterion) {
    let input = 12345;

    c.bench_function("Math Approach - Medium", |b| {
        b.iter(|| {
            let _ = base_iterative::reverse_integer(black_box(input));
        })
    });

    c.bench_function("String Approach - Medium", |b| {
        b.iter(|| {
            let _ = string_conversion::reverse_integer(black_box(input));
        })
    });

    c.bench_function("Recursive Approach - Medium", |b| {
        b.iter(|| {
            let _ = base_recursive::reverse_integer(black_box(input));
        })
    });

    c.bench_function("Divide and Conquer - Medium", |b| {
        b.iter(|| {
            let _ = divide_and_conquer::reverse_integer(black_box(input));
        })
    });
}

fn bench_large(c: &mut Criterion) {
    let input = generate_large_input(9);

    c.bench_function("Math Approach - Large", |b| {
        b.iter(|| {
            let _ = base_iterative::reverse_integer(black_box(input));
        })
    });

    c.bench_function("String Approach - Large", |b| {
        b.iter(|| {
            let _ = string_conversion::reverse_integer(black_box(input));
        })
    });

    c.bench_function("Recursive Approach - Large", |b| {
        b.iter(|| {
            let _ = base_recursive::reverse_integer(black_box(input));
        })
    });

    c.bench_function("Divide and Conquer - Large", |b| {
        b.iter(|| {
            let _ = divide_and_conquer::reverse_integer(black_box(input));
        })
    });
}

criterion_group!(benches, bench_medium, bench_large);
criterion_main!(benches);
