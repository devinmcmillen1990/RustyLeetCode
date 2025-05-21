use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use RustyLeetCode::dynamic_programming::medium::interleaving_string::{
    bottom_up_dp, space_optimized_dp, top_down_memoization,
};

fn bench_interleaving_string_med_size_input(c: &mut Criterion) {
    let (m_s1, m_s2, m_s3) = medium_input();

    c.bench_function("bottom_up - medium", |b| {
        b.iter(|| {
            bottom_up_dp::is_interleave(
                black_box(m_s1.clone()),
                black_box(m_s2.clone()),
                black_box(m_s3.clone()),
            )
        })
    });

    c.bench_function("top_down - medium", |b| {
        b.iter(|| {
            top_down_memoization::is_interleave(
                black_box(m_s1.clone()),
                black_box(m_s2.clone()),
                black_box(m_s3.clone()),
            )
        })
    });

    c.bench_function("space_optimized - medium", |b| {
        b.iter(|| {
            space_optimized_dp::is_interleave(
                black_box(m_s1.clone()),
                black_box(m_s2.clone()),
                black_box(m_s3.clone()),
            )
        })
    });
}

fn bench_interleaving_string_large_size_input(c: &mut Criterion) {
    let (l_s1, l_s2, l_s3) = large_input();

    c.bench_function("bottom_up - large", |b| {
        b.iter(|| {
            bottom_up_dp::is_interleave(
                black_box(l_s1.clone()),
                black_box(l_s2.clone()),
                black_box(l_s3.clone()),
            )
        })
    });

    c.bench_function("top_down - large", |b| {
        b.iter(|| {
            top_down_memoization::is_interleave(
                black_box(l_s1.clone()),
                black_box(l_s2.clone()),
                black_box(l_s3.clone()),
            )
        })
    });

    c.bench_function("space_optimized - large", |b| {
        b.iter(|| {
            space_optimized_dp::is_interleave(
                black_box(l_s1.clone()),
                black_box(l_s2.clone()),
                black_box(l_s3.clone()),
            )
        })
    });
}

fn medium_input() -> (String, String, String) {
    let s1 = "abc".repeat(50); // 150 chars
    let s2 = "xyz".repeat(50); // 150 chars
    let mut s3 = String::new();
    for _i in 0..50 {
        s3.push_str("a");
        s3.push_str("x");
        s3.push_str("b");
        s3.push_str("y");
        s3.push_str("c");
        s3.push_str("z");
    }
    (s1, s2, s3) // 300 total characters
}

fn large_input() -> (String, String, String) {
    let s1 = "abc".repeat(200); // 600 chars
    let s2 = "xyz".repeat(200); // 600 chars
    let mut s3 = String::new();
    for _i in 0..200 {
        s3.push_str("a");
        s3.push_str("x");
        s3.push_str("b");
        s3.push_str("y");
        s3.push_str("c");
        s3.push_str("z");
    }
    (s1, s2, s3) // 1200 total characters
}

criterion_group!(
    benches,
    bench_interleaving_string_med_size_input,
    bench_interleaving_string_large_size_input
);
criterion_main!(benches);
