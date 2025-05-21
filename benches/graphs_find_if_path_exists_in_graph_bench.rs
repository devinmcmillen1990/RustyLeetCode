use criterion::{criterion_group, criterion_main, Criterion};
use rand::{Rng, SeedableRng};
use std::hint::black_box;
use RustyLeetCode::graphs::easy::find_if_path_exists_in_graph::{
    bfs, dfs, union_find_disjoint_set_union,
};

fn bench_valid_path_med_size_input(c: &mut Criterion) {
    let (mn, med_edges, med_src, med_dst) = medium_input();

    c.bench_function("DFS - Medium", |b| {
        b.iter(|| {
            dfs::valid_path(
                black_box(mn),
                black_box(med_edges.clone()),
                black_box(med_src),
                black_box(med_dst),
            )
        })
    });

    c.bench_function("BFS - Medium", |b| {
        b.iter(|| {
            bfs::valid_path(
                black_box(mn),
                black_box(med_edges.clone()),
                black_box(med_src),
                black_box(med_dst),
            )
        })
    });

    c.bench_function("Union-Find - Medium", |b| {
        b.iter(|| {
            union_find_disjoint_set_union::valid_path(
                black_box(mn),
                black_box(med_edges.clone()),
                black_box(med_src),
                black_box(med_dst),
            )
        })
    });
}

fn bench_valid_path_large_size_input(c: &mut Criterion) {
    let (ln, large_edges, large_src, large_dst) = large_input();

    c.bench_function("DFS - Large", |b| {
        b.iter(|| {
            dfs::valid_path(
                black_box(ln),
                black_box(large_edges.clone()),
                black_box(large_src),
                black_box(large_dst),
            )
        })
    });

    c.bench_function("BFS - Large", |b| {
        b.iter(|| {
            bfs::valid_path(
                black_box(ln),
                black_box(large_edges.clone()),
                black_box(large_src),
                black_box(large_dst),
            )
        })
    });

    c.bench_function("Union-Find - Large", |b| {
        b.iter(|| {
            union_find_disjoint_set_union::valid_path(
                black_box(ln),
                black_box(large_edges.clone()),
                black_box(large_src),
                black_box(large_dst),
            )
        })
    });
}

fn medium_input() -> (i32, Vec<Vec<i32>>, i32, i32) {
    // A chain of 1_000 nodes
    let n = 1_000;
    let edges = (0..n - 1).map(|i| vec![i, i + 1]).collect();
    let source = 0;
    let destination = n - 1;
    (n, edges, source, destination)
}

fn large_input() -> (i32, Vec<Vec<i32>>, i32, i32) {
    // Dense random graph of 10_000 nodes and ~50_000 edges
    let n = 10_000;
    let mut edges = Vec::with_capacity(50_000);
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    for _ in 0..50_000 {
        let u = rng.random_range(0..n);
        let v = rng.random_range(0..n);
        if u != v {
            edges.push(vec![u, v]);
        }
    }

    let source = 0;
    let destination = n - 1;
    (n, edges, source, destination)
}

criterion_group!(
    benches,
    bench_valid_path_med_size_input,
    bench_valid_path_large_size_input
);
criterion_main!(benches);
