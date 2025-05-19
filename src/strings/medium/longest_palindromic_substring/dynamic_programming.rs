/*
Longest Palindromic Substring - Dynamic Programming
Approach:           * Define a DP table dp[i][j] where dp[i][j] = true if the substring s[i..=j] is a 
                      palindrome.
                    * Populate the table by expanding outward and checking the characters.
                    * Track the longest palindrome.

Time Complexity:    O(n²)
Space Complexity:   O(n²) — For the DP table.
*/
pub fn longest_palindromic_substring(s: String) -> String {
    let s_len = s.len();

    if s_len < 2 { return s; }

    let mut dp_table = vec![vec![false; s_len]; s_len];
    let chars: Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut max_len = 1;

    // Since every individual character is a palindrome, set each character cell true.
    for i in 0..s_len {
        dp_table[i][i] = true;
    }

    for length in 2..=s_len {
        for i in 0..=s_len - length {
            let j = i + length - 1;

            if chars[i] == chars[j] {
                if length == 2 || dp_table[i + 1][j - 1] {
                    dp_table[i][j] = true;
                    if length > max_len {
                        max_len = length;
                        start = i;
                    }
                }
            }
        }
    }

    s[start..start + max_len].to_string()
}