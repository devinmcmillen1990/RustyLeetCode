/*
Longest Palindromic Substring - Center Expansion
Approach:           * For each character (and each pair of characters), consider it as a center and expand
                      outwards to find the longest palindrome.
                    * This approach handles both odd and even length palindromes.

Time Complexity:    O(n²)
Space Complexity:   O(1)
*/
pub fn longest_palindromic_substring(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let s_len = s.len();
    let mut start = 0;
    let mut max_len = 0;

    for i in 0..s_len {
        let len1 = expand_from_center(&chars, i, i); // Odd-Length Palindrom
        let len2 = expand_from_center(&chars, i, i + 1); // Even-Length Palindrome

        let current_len = len1.max(len2);
        if current_len > max_len {
            max_len = current_len;
            start = i - (current_len - 1) / 2;
        }
    }

    s[start..start + max_len].to_string()
}

fn expand_from_center(chars: &[char], left: usize, right: usize) -> usize {
    let chars_len = chars.len();
    let mut left_runner = left as i32;
    let mut right_runner = right as i32;

    while left_runner >= 0
        && right_runner < chars_len as i32
        && chars[left_runner as usize] == chars[right_runner as usize]
    {
        left_runner -= 1;
        right_runner += 1;
    }

    (right_runner - left_runner - 1) as usize
}
