use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use RustyLeetCode::arrays::medium::median_of_two_sorted_arrays::{
    binary_search, merge_and_sort, two_pointers_merge,
};

/// Benchmark input sizes
const SMALL_NUMS1: &[i32] = &[1, 3];
const SMALL_NUMS2: &[i32] = &[2];

const MEDIUM_NUMS1: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const MEDIUM_NUMS2: &[i32] = &[11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

/// Large inputs with repeated values to simulate edge cases
fn generate_large_input(size: usize) -> Vec<i32> {
    (0..size).map(|i| (i % 1000) as i32).collect()
}

fn bench_small(c: &mut Criterion) {
    let nums1 = SMALL_NUMS1.to_vec();
    let nums2 = SMALL_NUMS2.to_vec();

    c.bench_function("Merge Sort - Small", |b| {
        b.iter(|| {
            let _ = merge_and_sort::median_of_two_sorted_arrays(
                black_box(nums1.clone()),
                black_box(nums2.clone()),
            );
        })
    });

    c.bench_function("Two Pointers - Small", |b| {
        b.iter(|| {
            let _ = two_pointers_merge::median_of_two_sorted_arrays(
                black_box(nums1.clone()),
                black_box(nums2.clone()),
            );
        })
    });

    c.bench_function("Binary Search - Small", |b| {
        b.iter(|| {
            let _ = binary_search::median_of_two_sorted_arrays(
                black_box(nums1.clone()),
                black_box(nums2.clone()),
            );
        })
    });
}

fn bench_medium(c: &mut Criterion) {
    let nums1 = MEDIUM_NUMS1.to_vec();
    let nums2 = MEDIUM_NUMS2.to_vec();

    c.bench_function("Merge Sort - Medium", |b| {
        b.iter(|| {
            let _ = merge_and_sort::median_of_two_sorted_arrays(
                black_box(nums1.clone()),
                black_box(nums2.clone()),
            );
        })
    });

    c.bench_function("Two Pointers - Medium", |b| {
        b.iter(|| {
            let _ = two_pointers_merge::median_of_two_sorted_arrays(
                black_box(nums1.clone()),
                black_box(nums2.clone()),
            );
        })
    });

    c.bench_function("Binary Search - Medium", |b| {
        b.iter(|| {
            let _ = binary_search::median_of_two_sorted_arrays(
                black_box(nums1.clone()),
                black_box(nums2.clone()),
            );
        })
    });
}

fn bench_large(c: &mut Criterion) {
    let nums1 = generate_large_input(50_000);
    let nums2 = generate_large_input(50_000);

    c.bench_function("Merge Sort - Large", |b| {
        b.iter(|| {
            let _ = merge_and_sort::median_of_two_sorted_arrays(
                black_box(nums1.clone()),
                black_box(nums2.clone()),
            );
        })
    });

    c.bench_function("Two Pointers - Large", |b| {
        b.iter(|| {
            let _ = two_pointers_merge::median_of_two_sorted_arrays(
                black_box(nums1.clone()),
                black_box(nums2.clone()),
            );
        })
    });

    c.bench_function("Binary Search - Large", |b| {
        b.iter(|| {
            let _ = binary_search::median_of_two_sorted_arrays(
                black_box(nums1.clone()),
                black_box(nums2.clone()),
            );
        })
    });
}

criterion_group!(benches, bench_small, bench_medium, bench_large);
criterion_main!(benches);
