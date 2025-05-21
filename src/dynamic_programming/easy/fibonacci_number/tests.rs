use crate::dynamic_programming::easy::fibonacci_number::{
    binets_formula_closed_form, bottom_up_dp, matrix_exponentiation, recursively,
    recursively_with_memoization, space_optimized_dp,
};

#[test]
fn test_fib_recursive() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = recursively::fib(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: fib({}) = {}, expected {}",
            i, case.n, result, case.expected
        );
    }
}

#[test]
fn test_fib_memoized() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = recursively_with_memoization::fib(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: fib({}) = {}, expected {}",
            i, case.n, result, case.expected
        );
    }
}

#[test]
fn test_fib_bottom_up_dp() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = bottom_up_dp::fib(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: fib({}) = {}, expected {}",
            i, case.n, result, case.expected
        );
    }
}

#[test]
fn test_fib_space_optimized_dp() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = space_optimized_dp::fib(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: fib({}) = {}, expected {}",
            i, case.n, result, case.expected
        );
    }
}

#[test]
fn test_fib_matrix_exponentiation() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = matrix_exponentiation::fib(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: fib({}) = {}, expected {}",
            i, case.n, result, case.expected
        );
    }
}

#[test]
fn test_fib_binets_formula_closed_form() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = binets_formula_closed_form::fib(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: fib({}) = {}, expected {}",
            i, case.n, result, case.expected
        );
    }
}

struct TestCase {
    n: i32,
    expected: i32,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase { n: 0, expected: 0 },
        TestCase { n: 1, expected: 1 },
        TestCase { n: 2, expected: 1 },
        TestCase { n: 3, expected: 2 },
        TestCase { n: 5, expected: 5 },
        TestCase {
            n: 10,
            expected: 55,
        },
        TestCase {
            n: 20,
            expected: 6765,
        },
        TestCase {
            n: 30,
            expected: 832040,
        },
    ]
}
