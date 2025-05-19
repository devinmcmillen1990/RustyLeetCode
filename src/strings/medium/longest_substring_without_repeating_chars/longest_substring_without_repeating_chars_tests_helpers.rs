#[cfg(test)]
pub struct TestCase {
    pub input: String,
    pub expected: i32,
}

#[cfg(test)]
pub fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            input: "abcabcbb".to_string(),
            expected: 3, // "abc"
        },
        TestCase {
            input: "bbbbb".to_string(),
            expected: 1, // "b"
        },
        TestCase {
            input: "pwwkew".to_string(),
            expected: 3, // "wke"
        },
        TestCase {
            input: "".to_string(),
            expected: 0, // Empty string
        },
        TestCase {
            input: "a".to_string(),
            expected: 1, // "a"
        },
        TestCase {
            input: "abcdef".to_string(),
            expected: 6, // "abcdef"
        },
        TestCase {
            input: "abca".to_string(),
            expected: 3, // "abc"
        },
        TestCase {
            input: "dvdf".to_string(),
            expected: 3, // "vdf"
        },
        TestCase {
            input: "anviaj".to_string(),
            expected: 5, // "nviaj"
        },
        TestCase {
            input: " ".to_string(),
            expected: 1, // " "
        },
        TestCase {
            input: "au".to_string(),
            expected: 2, // "au"
        },
    ]
}

/// Custom assertion function to verify output.
#[cfg(test)]
pub fn assert_length_of_longest_substring(actual: i32, expected: i32, case_index: usize) {
    assert_eq!(
        actual, expected,
        "Test case {} failed: Expected {}, got {}",
        case_index, expected, actual
    );
}