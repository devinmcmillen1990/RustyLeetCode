use crate::arrays::medium::permutations2::{
    iteratively_heap_with_hashset_dedup, lazy_yield_iterator::UniquePermutations,
    recursively_simple_eager_with_hashset, recursively_with_backtracking_with_hashset_dedup,
    recursively_with_backtracking_with_in_place_swaps_and_local_hashset,
    recursively_with_backtracking_with_vec_for_dedups,
};

#[test]
fn test_backtracking_current_path_memoization() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result =
            recursively_with_backtracking_with_vec_for_dedups::permute_unique(case.input.clone());
        assert_eq!(
            normalize(result),
            normalize(case.expected.clone()),
            "backtracking_with_current_path_memoization::permute_unique failed at case {}: input={:?}",
            i,
            case.input
        );
    }
}

#[test]
fn test_backtracking_hashset_dedup() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result =
            recursively_with_backtracking_with_hashset_dedup::permute_unique(case.input.clone());
        assert_eq!(
            normalize(result),
            normalize(case.expected.clone()),
            "recursively_with_backtracking_with_hashset_dedup::permute_unique failed at case {}: input={:?}",
            i,
            case.input
        );
    }
}

#[test]
fn test_backtracking_in_place_swaps_and_local_set() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result =
            recursively_with_backtracking_with_in_place_swaps_and_local_hashset::permute_unique(
                case.input.clone(),
            );
        assert_eq!(
            normalize(result),
            normalize(case.expected.clone()),
            "backtracking_with_in_place_swaps_and_local_hashset::permute_unique failed at case {}: input={:?}",
            i,
            case.input
        );
    }
}

#[test]
fn test_iterative_heap_with_hashset_dedup() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = iteratively_heap_with_hashset_dedup::permute_unique(case.input.clone());
        assert_eq!(
            normalize(result),
            normalize(case.expected.clone()),
            "iterative_heap_with_hashset_dedup::permute_unique failed at case {}: input={:?}",
            i,
            case.input
        );
    }
}

#[test]
fn test_lazy_yield_iterator() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result: Vec<_> = UniquePermutations::new(case.input.clone()).collect();
        assert_eq!(
            normalize(result),
            normalize(case.expected.clone()),
            "lazy_yield_iterator failed at case {}: input={:?}",
            i,
            case.input
        );
    }
}

#[test]
fn test_recursively_simple_eager_with_set() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = recursively_simple_eager_with_hashset::permute_unique(case.input.clone());
        assert_eq!(
            normalize(result),
            normalize(case.expected.clone()),
            "recursively_simple_eager_with_hashset::permute_unique failed at case {}: input={:?}",
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
    let mut expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
    expected.sort();

    vec![TestCase {
        input: vec![1, 1, 2],
        expected,
    }]
}

fn normalize(mut perms: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    perms.sort();
    perms
}
