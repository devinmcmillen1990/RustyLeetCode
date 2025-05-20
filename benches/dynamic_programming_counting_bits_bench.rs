use criterion::{criterion_group, criterion_main, Criterion};

// === 1. Shift & Mask DP (Optimal) ===
fn count_bits_shift_mask(n: i32) -> Vec<i32> {
    let mut result = vec![0; (n + 1) as usize];
    for i in 1..=n as usize {
        result[i] = result[i >> 1] + (i & 1) as i32;
    }
    result
}

// === 2. MSB Offset DP ===
fn count_bits_msb_offset(n: i32) -> Vec<i32> {
    let mut result = vec![0; (n + 1) as usize];
    let mut offset = 1;
    for i in 1..=n as usize {
        if offset * 2 == i {
            offset = i;
        }
        result[i] = 1 + result[i - offset];
    }
    result
}

fn bench_count_bits_medium_data_size(c: &mut Criterion) {
    let medium_n = 1_000;

    c.bench_function("count_bits_shift_mask_1000", |b| {
        b.iter(|| count_bits_shift_mask(medium_n))
    });

    c.bench_function("count_bits_msb_offset_1000", |b| {
        b.iter(|| count_bits_msb_offset(medium_n))
    });
}

fn bench_count_bits_large_data_size(c: &mut Criterion) {
    let large_n = 100_000;

    c.bench_function("count_bits_shift_mask_100000", |b| {
        b.iter(|| count_bits_shift_mask(large_n))
    });

    c.bench_function("count_bits_msb_offset_100000", |b| {
        b.iter(|| count_bits_msb_offset(large_n))
    });
}

criterion_group!(
    benches,
    bench_count_bits_medium_data_size,
    bench_count_bits_large_data_size
);
criterion_main!(benches);
