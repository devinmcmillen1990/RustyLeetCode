/*
Length of Longest Substring - Sliding Window ASCII
Approach:           * Use two pointers (left and right) to maintain a sliding window of unique
                      characters.
                    * If the input is limited to ASCII characters (256 unique chars), we can
                      use a fixed-size array INSTEAD of a HashMap.

Time Complexity:    O(n) — We traverse the string once.
Space Complexity:   O(1) (Fixed array size)
*/
pub fn length_of_longest_substring_sliding_window_ascii(s: String) -> i32 {
    let mut last_seen = [-1; 256];
    let mut left: i32 = 0;
    let mut max_len: i32 = 0;

    for (right, ch) in s.chars().enumerate() {
        let index = ch as usize;

        if last_seen[index] != -1 {
            left = left.max(last_seen[index] + 1);
        }

        // TODO: I'm concerned about this cast. See if there is a better solution to help performance.
        let right_cast = right as i32;
        last_seen[index] = right_cast;
        max_len = max_len.max(right_cast - left + 1);
    }

    max_len
}
