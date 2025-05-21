use crate::dynamic_programming::medium::interleaving_string::{
    bottom_up_dp, space_optimized_dp, top_down_memoization,
};

#[test]
fn test_is_interleave_bottom_up() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = bottom_up_dp::is_interleave(
            case.s1.to_string(),
            case.s2.to_string(),
            case.s3.to_string(),
        );
        assert_eq!(
            result, case.expected,
            "bottom_up failed at case {}: s1={}, s2={}, s3={}, expected {}, got {}",
            i, case.s1, case.s2, case.s3, case.expected, result
        );
    }
}

#[test]
fn test_is_interleave_top_down() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = top_down_memoization::is_interleave(
            case.s1.to_string(),
            case.s2.to_string(),
            case.s3.to_string(),
        );
        assert_eq!(
            result, case.expected,
            "top_down failed at case {}: s1={}, s2={}, s3={}, expected {}, got {}",
            i, case.s1, case.s2, case.s3, case.expected, result
        );
    }
}

#[test]
fn test_is_interleave_space_optimized() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = space_optimized_dp::is_interleave(
            case.s1.to_string(),
            case.s2.to_string(),
            case.s3.to_string(),
        );
        assert_eq!(
            result, case.expected,
            "space_optimized failed at case {}: s1={}, s2={}, s3={}, expected {}, got {}",
            i, case.s1, case.s2, case.s3, case.expected, result
        );
    }
}

#[cfg(test)]
struct TestCase {
    s1: &'static str,
    s2: &'static str,
    s3: &'static str,
    expected: bool,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            s1: "aab",
            s2: "axy",
            s3: "aaxaby",
            expected: true,
        },
        TestCase {
            s1: "abc",
            s2: "def",
            s3: "abdecf",
            expected: true,
        },
        TestCase {
            s1: "",
            s2: "",
            s3: "",
            expected: true,
        },
        TestCase {
            s1: "abc",
            s2: "",
            s3: "abc",
            expected: true,
        },
        TestCase {
            s1: "",
            s2: "def",
            s3: "def",
            expected: true,
        },
        TestCase {
            s1: "aabcc",
            s2: "dbbca",
            s3: "aadbbcbcac",
            expected: true,
        },
        TestCase {
            s1: "aabcc",
            s2: "dbbca",
            s3: "aadbbbaccc",
            expected: false,
        },
        TestCase {
            s1: "abc",
            s2: "123",
            s3: "a1b2c3",
            expected: true,
        },
        TestCase {
            s1: "abc",
            s2: "123",
            s3: "abc1234",
            expected: false,
        },
    ]
}
