/*
Longest Palindromic Substring - Brute Force
Approach:           * Iterate over every possible substring and check if it is a palindrome.
                    * Track the longest palindrome found.

Time Complexity:    O(n³)
Space Complexity:   O(1)
*/
pub fn longest_palindromic_substring(s: String) -> String {
    let mut max_len = 0;
    let mut longest = String::new(); // TODO: Probably will want to use the String Builder

    for i in 0..s.len() {
        for j in i..s.len() {
            let substring = &s[i..=j];
            if is_palindrome(substring) && substring.len() > max_len {
                max_len = substring.len();
                longest = substring.to_string();
            }
        }
    }

    longest
}

fn is_palindrome(substring: &str) -> bool {
    let chars: Vec<char> = substring.chars().collect();
    let substring_len = substring.len();
    for i in 0..substring_len / 2 {
        if chars[i] != chars[substring_len - i - 1] {
            return false;
        }
    }
    true
}
