use crate::graphs::easy::find_the_town_judge::{
    hashmap_based_trust_tracking, in_degree_out_degree_array, optimized_trust_score_approach,
    single_score_array_net_trust,
};

#[test]
fn test_find_judge_in_degree_out_degree() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = in_degree_out_degree_array::find_judge(case.n, case.trust.clone());
        assert_eq!(
            result, case.expected,
            "Test case {} failed: expected {}, got {}",
            i, case.expected, result
        );
    }
}

#[test]
fn test_find_judge_single_score_array_net_trust() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = single_score_array_net_trust::find_judge(case.n, case.trust.clone());
        assert_eq!(
            result, case.expected,
            "Test case {} failed: expected {}, got {}",
            i, case.expected, result
        );
    }
}

#[test]
fn test_find_judge_hashmap_based_trust_tracking() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = hashmap_based_trust_tracking::find_judge(case.n, case.trust.clone());
        assert_eq!(
            result, case.expected,
            "Test case {} failed: expected {}, got {}",
            i, case.expected, result
        );
    }
}

#[test]
fn test_find_judge_optimized_trust_score_approach() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = optimized_trust_score_approach::find_judge(case.n, case.trust.clone());
        assert_eq!(
            result, case.expected,
            "Test case {} failed: expected {}, got {}",
            i, case.expected, result
        );
    }
}

struct TestCase {
    n: i32,
    trust: Vec<Vec<i32>>,
    expected: i32,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            n: 2,
            trust: vec![vec![1, 2]],
            expected: 2,
        },
        TestCase {
            n: 3,
            trust: vec![vec![1, 3], vec![2, 3]],
            expected: 3,
        },
        TestCase {
            n: 3,
            trust: vec![vec![1, 3], vec![2, 3], vec![3, 1]],
            expected: -1,
        },
        TestCase {
            n: 1,
            trust: vec![],
            expected: 1,
        },
        TestCase {
            n: 3,
            trust: vec![],
            expected: -1,
        },
    ]
}
