use crate::dynamic_programming::easy::tribanacci_number::{
    bottom_up_dp, matrix_exponentiation, space_optimized_dp, top_down_memoization,
};

#[test]
fn test_tribonacci_top_down() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = top_down_memoization::tribonacci(case.input);
        assert_eq!(
            result, case.expected,
            "top_down failed at case {}: input={}, expected {}, got {}",
            i, case.input, case.expected, result
        );
    }
}

#[test]
fn test_tribonacci_bottom_up() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = bottom_up_dp::tribonacci(case.input);
        assert_eq!(
            result, case.expected,
            "bottom_up failed at case {}: input={}, expected {}, got {}",
            i, case.input, case.expected, result
        );
    }
}

#[test]
fn test_tribonacci_space_optimized() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = space_optimized_dp::tribonacci(case.input);
        assert_eq!(
            result, case.expected,
            "space_optimized failed at case {}: input={}, expected {}, got {}",
            i, case.input, case.expected, result
        );
    }
}

#[test]
fn test_tribonacci_matrix_expo() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = matrix_exponentiation::tribonacci(case.input);
        assert_eq!(
            result, case.expected,
            "matrix_expo failed at case {}: input={}, expected {}, got {}",
            i, case.input, case.expected, result
        );
    }
}

#[cfg(test)]
struct TestCase {
    input: i32,
    expected: i32,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            input: 0,
            expected: 0,
        },
        TestCase {
            input: 1,
            expected: 1,
        },
        TestCase {
            input: 2,
            expected: 1,
        },
        TestCase {
            input: 3,
            expected: 2,
        },
        TestCase {
            input: 4,
            expected: 4,
        },
        TestCase {
            input: 5,
            expected: 7,
        },
        TestCase {
            input: 10,
            expected: 149,
        },
        TestCase {
            input: 20,
            expected: 66012,
        },
        TestCase {
            input: 25,
            expected: 1389537,
        },
        TestCase {
            input: 37,
            expected: 2082876103,
        },
    ]
}
