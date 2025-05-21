use crate::dynamic_programming::easy::counting_bits::{
    brute_force, dynamic_programming, most_significant_bit_pattern,
};

#[test]
fn test_count_bits_brute_force() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = brute_force::count_bits(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: n = {}, expected {:?}, got {:?}",
            i, case.n, case.expected, result
        );
    }
}

#[test]
fn test_count_bits_dynamic_programming() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = dynamic_programming::count_bits(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: n = {}, expected {:?}, got {:?}",
            i, case.n, case.expected, result
        );
    }
}

#[test]
fn test_count_bits_most_significant_bit_pattern() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = most_significant_bit_pattern::count_bits(case.n);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: n = {}, expected {:?}, got {:?}",
            i, case.n, case.expected, result
        );
    }
}

struct TestCase {
    n: i32,
    expected: Vec<i32>,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            n: 0,
            expected: vec![0],
        },
        TestCase {
            n: 1,
            expected: vec![0, 1],
        },
        TestCase {
            n: 2,
            expected: vec![0, 1, 1],
        },
        TestCase {
            n: 5,
            expected: vec![0, 1, 1, 2, 1, 2],
        },
        TestCase {
            n: 10,
            expected: vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2],
        },
        TestCase {
            n: 15,
            expected: vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4],
        },
    ]
}
