use std::collections::HashSet;

/*
Length of Longest Substring - Brute Force
Approach:           * Iterate over all possible substrings and check each one for unique
                      characters
                    * Track the max length of all valid substrings

Time Complexity:    O(n²)
Space Complexity:   O(n) (for the HashSet)
*/
pub fn length_of_longest_substring_brute_force(s: String) -> i32 {
    let mut max_length: i32 = 0;

    for i in 0..s.len() {
        for j in i + 1..=s.len() {
            let substring = &s[i..j];
            if has_unique_chars(substring) {
                max_length = max_length.max(substring.len() as i32);
            }
        }
    }

    max_length
}

fn has_unique_chars(substring: &str) -> bool {
    let mut seen = HashSet::new();
    for ch in substring.chars() {
        if !seen.insert(ch) {
            return false;
        }
    }
    true
}
