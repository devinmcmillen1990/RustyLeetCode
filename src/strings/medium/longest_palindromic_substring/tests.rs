use super::{brute_force, center_expansion, dynamic_programming, manachers_algorithm};

#[test]
fn test_longest_palindrome_brute_force() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = brute_force::longest_palindromic_substring(case.input);
        assert_longest_palindromic_substring(result, case.expected, index);
    }
}

#[test]
fn test_longest_palindrome_center_expand() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = center_expansion::longest_palindromic_substring(case.input);
        assert_longest_palindromic_substring(result, case.expected, index);
    }
}

#[test]
fn test_longest_palindrome_dynamic_programming() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = dynamic_programming::longest_palindromic_substring(case.input);
        assert_longest_palindromic_substring(result, case.expected, index);
    }
}

#[test]
fn test_longest_palindrome_manachers_algorithm() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = manachers_algorithm::longest_palindromic_substring(case.input);
        assert_longest_palindromic_substring(result, case.expected, index);
    }
}

#[cfg(test)]
pub struct TestCase {
    pub input: String,
    pub expected: String,
}

#[cfg(test)]
pub fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            input: "babad".to_string(),
            expected: "bab".to_string(), // "aba" is also a valid answer
        },
        TestCase {
            input: "cbbd".to_string(),
            expected: "bb".to_string(),
        },
        TestCase {
            input: "a".to_string(),
            expected: "a".to_string(), // Single character is a palindrome
        },
        TestCase {
            input: "".to_string(),
            expected: "".to_string(), // Empty string
        },
        TestCase {
            input: "racecar".to_string(),
            expected: "racecar".to_string(), // Entire string is a palindrome
        },
        TestCase {
            input: "abcdefg".to_string(),
            expected: "a".to_string(), // No repeating characters
        },
        TestCase {
            input: "noonmadamracecar".to_string(),
            expected: "racecar".to_string(), // Multiple palindromes, largest is "racecar"
        },
        TestCase {
            input: "aabbcc".to_string(),
            expected: "aa".to_string(), // Multiple 2-char palindromes, "aa" is the first
        },
        TestCase {
            input: "xyzzy".to_string(),
            expected: "yzzy".to_string(),
        },
        TestCase {
            input: "abcdefgh".to_string(),
            expected: "a".to_string(), // Single character palindrome
        },
        TestCase {
            input: "tattarrattat".to_string(),
            expected: "tattarrattat".to_string(), // Entire string is a palindrome
        },
    ]
}

/// Custom assertion function to verify output.
#[cfg(test)]
pub fn assert_longest_palindromic_substring(actual: String, expected: String, case_index: usize) {
    assert!(
        actual == expected || actual.chars().rev().collect::<String>() == expected,
        "Test case {} failed: Expected {}, got {}",
        case_index,
        expected,
        actual
    );
}
