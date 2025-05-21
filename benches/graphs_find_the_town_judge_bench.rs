use criterion::{criterion_group, criterion_main, Criterion};
use RustyLeetCode::graphs::easy::find_the_town_judge::{
    hashmap_based_trust_tracking, in_degree_out_degree_array, optimized_trust_score_approach,
    single_score_array_net_trust,
};

fn generate_trust_data(n: usize) -> Vec<Vec<i32>> {
    let mut trust = vec![];
    for i in 1..n {
        trust.push(vec![i as i32, n as i32]); // everyone trusts person n
    }
    trust
}

fn bench_find_judge_medium_data_size(c: &mut Criterion) {
    let medium_n = 1_000;
    let medium_data = generate_trust_data(medium_n);

    c.bench_function("Find Town Judge - Medium - In-Degree/Out-Degree", |b| {
        b.iter(|| in_degree_out_degree_array::find_judge(medium_n as i32, medium_data.clone()))
    });

    c.bench_function(
        "Find Town Judge - Medium - Single Score Array (Net Trust)",
        |b| {
            b.iter(|| {
                single_score_array_net_trust::find_judge(medium_n as i32, medium_data.clone())
            })
        },
    );

    c.bench_function(
        "Find Town Judge - Medium - HashMap Based Trust Tracking",
        |b| {
            b.iter(|| {
                hashmap_based_trust_tracking::find_judge(medium_n as i32, medium_data.clone())
            })
        },
    );

    c.bench_function(
        "Find Town Judge - Medium - Optimized Trust Score Approach",
        |b| {
            b.iter(|| {
                optimized_trust_score_approach::find_judge(medium_n as i32, medium_data.clone())
            })
        },
    );
}

fn bench_find_judge_large_data_size(c: &mut Criterion) {
    let large_n = 100_000;
    let large_data = generate_trust_data(large_n);

    c.bench_function("Find Town Judge - Large - In-Degree/Out-Degree", |b| {
        b.iter(|| in_degree_out_degree_array::find_judge(large_n as i32, large_data.clone()))
    });

    c.bench_function(
        "Find Town Judge - Large - Single Score Array (Net Trust)",
        |b| b.iter(|| single_score_array_net_trust::find_judge(large_n as i32, large_data.clone())),
    );

    c.bench_function(
        "Find Town Judge - Large - HashMap Based Trust Tracking",
        |b| b.iter(|| hashmap_based_trust_tracking::find_judge(large_n as i32, large_data.clone())),
    );

    c.bench_function(
        "Find Town Judge - Large - Optimized Trust Score Approach",
        |b| {
            b.iter(|| {
                optimized_trust_score_approach::find_judge(large_n as i32, large_data.clone())
            })
        },
    );
}

criterion_group!(
    benches,
    bench_find_judge_medium_data_size,
    bench_find_judge_large_data_size
);
criterion_main!(benches);
