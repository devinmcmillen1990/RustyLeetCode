use crate::math::easy::nim_game::{bottom_up_dp, mathy};

#[test]
fn test_can_win_nim_mathy() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = mathy::can_win_nim(case.input);
        assert_eq!(
            result, case.expected,
            "mathy failed at test case {}: input {}, expected {}, got {}",
            index, case.input, case.expected, result
        );
    }
}

#[test]
fn test_can_win_nim_bottom_up() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = bottom_up_dp::can_win_nim(case.input);
        assert_eq!(
            result, case.expected,
            "bottom_up failed at test case {}: input {}, expected {}, got {}",
            index, case.input, case.expected, result
        );
    }
}

#[cfg(test)]
struct TestCase {
    input: i32,
    expected: bool,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            input: 1,
            expected: true,
        },
        TestCase {
            input: 2,
            expected: true,
        },
        TestCase {
            input: 3,
            expected: true,
        },
        TestCase {
            input: 4,
            expected: false,
        },
        TestCase {
            input: 5,
            expected: true,
        },
        TestCase {
            input: 6,
            expected: true,
        },
        TestCase {
            input: 7,
            expected: true,
        },
        TestCase {
            input: 8,
            expected: false,
        },
        TestCase {
            input: 12,
            expected: false,
        },
        TestCase {
            input: 13,
            expected: true,
        },
        TestCase {
            input: 20,
            expected: false,
        },
        TestCase {
            input: 21,
            expected: true,
        },
        TestCase {
            input: 100,
            expected: false,
        },
        TestCase {
            input: 101,
            expected: true,
        },
    ]
}
