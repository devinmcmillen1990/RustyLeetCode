use crate::dynamic_programming::hard::number_of_ways_to_rearrange_sticks_with_k_visible::{
    bottom_up_dp, matrix_exponentiation, top_down_memoization,
};

#[test]
fn test_rearrange_sticks_bottom_up() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = bottom_up_dp::rearrange_sticks(case.n, case.k);
        assert_eq!(
            result, case.expected,
            "bottom_up failed at case {}: input=({}, {}), expected {}, got {}",
            i, case.n, case.k, case.expected, result
        );
    }
}

#[test]
fn test_rearrange_sticks_top_down() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = top_down_memoization::rearrange_sticks(case.n, case.k);
        assert_eq!(
            result, case.expected,
            "top_down failed at case {}: input=({}, {}), expected {}, got {}",
            i, case.n, case.k, case.expected, result
        );
    }
}

#[test]
fn test_rearrange_sticks_matrix_expo() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = matrix_exponentiation::rearrange_sticks(case.n, case.k);
        assert_eq!(
            result, case.expected,
            "matrix_expo failed at case {}: input=({}, {}), expected {}, got {}",
            i, case.n, case.k, case.expected, result
        );
    }
}

#[cfg(test)]
struct TestCase {
    n: i32,
    k: i32,
    expected: i32,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            n: 3,
            k: 2,
            expected: 3,
        },
        TestCase {
            n: 3,
            k: 1,
            expected: 2,
        },
        TestCase {
            n: 3,
            k: 3,
            expected: 1,
        },
        TestCase {
            n: 4,
            k: 2,
            expected: 11,
        },
        TestCase {
            n: 5,
            k: 2,
            expected: 50,
        },
        TestCase {
            n: 20,
            k: 5,
            expected: 745534512 ,
        },
    ]
}
