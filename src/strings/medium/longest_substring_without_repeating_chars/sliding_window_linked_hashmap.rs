use std::collections::HashMap;

/*
Length of Longest Substring - Linked Hash Map
Approach:           * Use two pointers (left and right) to maintain a sliding window of unique
                      characters.
                    * Use a LinkedHashMap to maintain the insertion order and remove stale entries.

Time Complexity:    O(n) — We traverse the string once.
Space Complexity:   O(n)
*/
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_map = HashMap::new();
    let mut max_len: i32 = 0;
    let mut left: i32 = 0;

    for (right, ch) in s.chars().enumerate() {
        if let Some(&index) = char_map.get(&ch) {
            // TODO: Concerned about this cast. See if there is a more organized/performant solution
            left = left.max((index as i32) + 1);
        }

        // TODO: Concerned about this cast. See if there is a more organized/performant solution
        let right_casted = right as i32;
        char_map.insert(ch, right_casted);
        max_len = max_len.max(right_casted - left + 1);
    }

    max_len
}
