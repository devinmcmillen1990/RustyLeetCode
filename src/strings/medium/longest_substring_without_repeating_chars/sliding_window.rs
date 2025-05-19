use std::collections::HashMap;

/*
Length of Longest Substring - Sliding Window
Approach:           * Use two pointers (left and right) to maintain a sliding window of unique
                      characters.
                    * Utilize a HashMap to track character indices for quick lookups.

Time Complexity:    O(n) — We traverse the string once.
Space Complexity:   O(n) — HashMap stores character indices.
*/
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_map = HashMap::new();
    let mut left: i32 = 0;
    let mut max_len: i32 = 0;

    for (right, character) in s.chars().enumerate() {
        if let Some(&prev_index) = char_map.get(&character) {
            // TODO: Concerned about this cast. See if there is a more organized/performant solution
            left = left.max((prev_index as i32) + 1); // Move the left pointer to the right of the last occurrence
        }

        char_map.insert(character, right);
        // TODO: Concerned about this cast. See if there is a more organized/performant solution
        max_len = max_len.max((right as i32) - left + 1);
    }

    max_len
}
