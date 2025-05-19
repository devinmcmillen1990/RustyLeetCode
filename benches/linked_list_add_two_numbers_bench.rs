use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use RustyLeetCode::{
    linked_list::medium::{
        add_two_numbers::add_two_numbers_iteratively::add_two_nums_iteratively,
        add_two_numbers::add_two_numbers_iteratively_loop_unroll::add_two_nums_iteratively_loop_unroll,
        add_two_numbers::add_two_numbers_iteratively_mem_aligned::add_two_numbers_iteratively_mem_aligned,
        add_two_numbers::add_two_numbers_iteratively_mem_pooling::add_two_numbers_iteratively_mem_pooling,
        add_two_numbers::add_two_numbers_recursively::add_two_nums_recursively,
    },
    structs::list_node::{AlignedListNode, ListNode},
};

fn bench_medium(c: &mut Criterion) {
    let l1 = generate_medium_list(1000);
    let l2 = generate_medium_list(1000);

    let aligned_l1 = convert_to_aligned(l1.clone());
    let aligned_l2 = convert_to_aligned(l2.clone());

    c.bench_function("Iterative - Medium", |b| {
        b.iter(|| {
            let _ = add_two_nums_iteratively(black_box(l1.clone()), black_box(l2.clone()));
        })
    });

    c.bench_function("Recursive - Medium", |b| {
        b.iter(|| {
            let _ = add_two_nums_recursively(black_box(l1.clone()), black_box(l2.clone()));
        })
    });

    c.bench_function("Unrolled - Medium", |b| {
        b.iter(|| {
            let _ =
                add_two_nums_iteratively_loop_unroll(black_box(l1.clone()), black_box(l2.clone()));
        })
    });

    c.bench_function("Memory Aligned - Medium", |b| {
        b.iter(|| {
            let _ = add_two_numbers_iteratively_mem_aligned(
                black_box(aligned_l1.clone()),
                black_box(aligned_l2.clone()),
            );
        })
    });

    c.bench_function("Memory Pooling - Medium", |b| {
        b.iter(|| {
            let _ = add_two_numbers_iteratively_mem_pooling(
                black_box(l1.clone()),
                black_box(l2.clone()),
            );
        })
    });
}

/// Benchmark for large-sized numbers
fn bench_large(c: &mut Criterion) {
    // TODO: Making the large_list too large causes a crash. See what we can do to get the numbers larger.
    let l1 = generate_large_list(12_000);
    let l2 = generate_large_list(12_000);

    //// Associated with the Aligned-Memory Bench tests that aren't implemented for the large dataset benchmark tests.
    // let aligned_l1 = convert_to_aligned(l1.clone());
    // let aligned_l2 = convert_to_aligned(l2.clone());

    c.bench_function("Iterative - Large", |b| {
        b.iter(|| {
            let _ = add_two_nums_iteratively(black_box(l1.clone()), black_box(l2.clone()));
        })
    });

    //// Not working with large number datasets.
    // c.bench_function("Recursive - Large", |b| {
    //     b.iter(|| {
    //         let _ = add_two_nums_recursively(black_box(l1.clone()), black_box(l2.clone()));
    //     })
    // });

    c.bench_function("Unrolled - Large", |b| {
        b.iter(|| {
            let _ =
                add_two_nums_iteratively_loop_unroll(black_box(l1.clone()), black_box(l2.clone()));
        })
    });

    //// Not working with large number datasets.
    // c.bench_function("Memory Aligned - Large", |b| {
    //     b.iter(|| {
    //         let _ = add_two_numbers_iteratively_mem_aligned(
    //             black_box(aligned_l1.clone()),
    //             black_box(aligned_l2.clone()),
    //         );
    //     })
    // });

    c.bench_function("Memory Pooling - Large", |b| {
        b.iter(|| {
            let _ = add_two_numbers_iteratively_mem_pooling(
                black_box(l1.clone()),
                black_box(l2.clone()),
            );
        })
    });
}

criterion_group!(benches, bench_medium, bench_large);

criterion_main!(benches);

fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut current = &mut dummy_head;

    for &val in &vec {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }

    dummy_head.next
}

/// Generate a medium-sized list of length `size`.
fn generate_medium_list(size: usize) -> Option<Box<ListNode>> {
    let mut list = vec![];
    for i in 0..size {
        list.push(i as i32 % 10);
    }
    vec_to_list(list)
}

/// Generate a large-sized list of length `size`.
fn generate_large_list(size: usize) -> Option<Box<ListNode>> {
    let mut list = vec![];
    for i in 0..size {
        list.push((i * 2) as i32 % 10);
    }
    vec_to_list(list)
}

/// Converts a `ListNode` to an `AlignedListNode` iteratively to avoid recursion stack overflow.
fn convert_to_aligned(node: Option<Box<ListNode>>) -> Option<Box<AlignedListNode>> {
    let mut dummy_head = Box::new(AlignedListNode::new(0));
    let mut current = &mut dummy_head;

    let mut node = node; // Rebind to mutable
    while let Some(mut n) = node {
        let new_node = Box::new(AlignedListNode::new(n.val));
        current.next = Some(new_node);
        current = current.next.as_mut().unwrap();
        node = n.next.take(); // Move to the next node
    }

    dummy_head.next
}
