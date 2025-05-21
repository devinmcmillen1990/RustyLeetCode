use crate::math::medium::reverse_integer::{
    iteratively, recursively, divide_and_conquer, string_conversion,
};

#[test]
fn test_reverse_integer_math() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = iteratively::reverse_integer(case.input);
        assert_reverse_integer(result, case.expected, index);
    }
}

#[test]
fn test_reverse_integer_string() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = string_conversion::reverse_integer(case.input);
        assert_reverse_integer(result, case.expected, index);
    }
}

#[test]
fn test_reverse_integer_recursive() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = recursively::reverse_integer(case.input);
        assert_reverse_integer(result, case.expected, index);
    }
}

#[test]
fn test_reverse_integer_divide_and_conquer() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = divide_and_conquer::reverse_integer(case.input);
        assert_reverse_integer(result, case.expected, index);
    }
}

#[cfg(test)]
pub struct TestCase {
    pub input: i32,
    pub expected: i32,
}

#[cfg(test)]
pub fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            input: 123,
            expected: 321,
        },
        TestCase {
            input: -123,
            expected: -321,
        },
        TestCase {
            input: 120,
            expected: 21,
        },
        TestCase {
            input: 0,
            expected: 0,
        },
        TestCase {
            input: 1534236469,
            expected: 0, // Overflow case
        },
        TestCase {
            input: -2147483648,
            expected: 0, // Underflow case
        },
        TestCase {
            input: -10,
            expected: -1,
        },
        TestCase {
            input: 1000000003,
            expected: 0, // Overflow when reversed
        },
        TestCase {
            input: 1463847412,
            expected: 2147483641,
        },
        TestCase {
            input: -1463847412,
            expected: -2147483641,
        },
    ]
}

#[cfg(test)]
pub fn assert_reverse_integer(actual: i32, expected: i32, case_index: usize) {
    assert_eq!(
        actual, expected,
        "Test case {} failed: Expected {}, got {}",
        case_index, expected, actual
    );
}
