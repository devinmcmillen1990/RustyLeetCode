use crate::arrays::medium::permutations::{
    in_place_swapping, iteratively_heap, recursively_simple_eager, recursively_with_backtracking,
};

#[test]
fn test_permute_simple_eager() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = recursively_simple_eager::permute(case.input.clone());
        assert_eq!(
            normalize(result),
            normalize(case.expected.clone()),
            "heap algorithm failed at case {}: input={:?}",
            i,
            case.input
        );
    }
}

#[test]
fn test_permute_backtracking() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = recursively_with_backtracking::permute(case.input.clone());
        assert_eq!(
            normalize(result),
            normalize(case.expected.clone()),
            "backtracking failed at case {}: input={:?}",
            i,
            case.input
        );
    }
}

#[test]
fn test_permute_inplace_swap() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = in_place_swapping::permute(case.input.clone());
        assert_eq!(
            normalize(result),
            normalize(case.expected.clone()),
            "in-place swap failed at case {}: input={:?}",
            i,
            case.input
        );
    }
}

#[test]
fn test_permute_heap() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = iteratively_heap::permute(case.input.clone());
        assert_eq!(
            normalize(result),
            normalize(case.expected.clone()),
            "heap algorithm failed at case {}: input={:?}",
            i,
            case.input
        );
    }
}

#[cfg(test)]
struct TestCase {
    input: Vec<i32>,
    expected: Vec<Vec<i32>>,
}

fn get_test_cases() -> Vec<TestCase> {
    let mut expected = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];
    expected.sort();

    vec![TestCase {
        input: vec![1, 2, 3],
        expected,
    }]
}

fn normalize(mut perms: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    perms.sort();
    perms
}
