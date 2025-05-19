use super::{
    longest_substring_without_repeating_chars_sliding_window_linked_hashmap::length_of_longest_substring_linked_hash_map,
    longest_substring_without_repeating_chars_tests_helpers::{
        assert_length_of_longest_substring, get_test_cases,
    },
};

#[test]
fn test_length_of_longest_substring_brute_force() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = length_of_longest_substring_linked_hash_map(case.input);
        assert_length_of_longest_substring(result, case.expected, index);
    }
}
