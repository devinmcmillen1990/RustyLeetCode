use crate::dynamic_programming::easy::climbing_stairs::{
    bottom_up_dp_array, optimized_dp_two_vars, recursively_with_memoization,
};

#[test]
fn test_climb_stairs_recursively_with_memoization() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = recursively_with_memoization::climb_stairs(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: n = {}, expected {}, got {}",
            i, case.n, case.expected, result
        );
    }
}

#[test]
fn test_climb_stairs_optimized_dp_two_vars() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = optimized_dp_two_vars::climb_stairs(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: n = {}, expected {}, got {}",
            i, case.n, case.expected, result
        );
    }
}

struct TestCase {
    n: i32,
    expected: i32,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase { n: 1, expected: 1 },
        TestCase { n: 2, expected: 2 },
        TestCase { n: 3, expected: 3 },
        TestCase { n: 4, expected: 5 },
        TestCase { n: 5, expected: 8 },
        TestCase {
            n: 10,
            expected: 89,
        },
        TestCase {
            n: 20,
            expected: 10946,
        },
    ]
}

#[test]
fn test_climb_stairs_bottom_up_dp_array() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = bottom_up_dp_array::climb_stairs(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: n = {}, expected {}, got {}",
            i, case.n, case.expected, result
        );
    }
}
