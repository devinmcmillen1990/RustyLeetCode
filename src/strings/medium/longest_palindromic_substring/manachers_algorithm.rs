/*
Longest Palindromic Substring - Manacher's Algorithm
Approach:           * Transform the string to handle even-length palindromes using special characters.
                    * Use a center expansion technique with additional tracking arrays to maintain
                      palindrome boundaries.
                    * This algorithm achieves linear time complexity by leveraging previously computed
                      information.

Time Complexity:    O(n)
Space Complexity:   O(n)
*/
pub fn longest_palindromic_substring(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let s_len = chars.len();

    if s_len < 2 {
        return s;
    }

    // Transform the string with '#' characters
    let mut processed = vec!['#'; 2 * s_len + 1];
    for i in 0..s_len {
        processed[2 * i + 1] = chars[i];
    }

    let mut center = 0;
    let mut right = 0;
    let mut max_len = 0;
    let mut start = 0;
    let mut p = vec![0; processed.len()];

    for i in 0..processed.len() {
        let mirror = 2 * center as isize - i as isize;

        if i < right {
            p[i] = p[mirror as usize].min(right - i);
        }

        // Expand around center `i`
        while (i as isize + p[i] as isize + 1) < processed.len() as isize
            && (i as isize - p[i] as isize - 1) >= 0
            && processed[(i + p[i] + 1) as usize] == processed[(i - p[i] - 1) as usize]
        {
            p[i] += 1;
        }

        // Update the right boundary and center
        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }

        // Update maximum length and starting index
        if p[i] > max_len {
            max_len = p[i];
            start = (i - p[i]) / 2;
        }
    }

    s[start..start + max_len].to_string()
}
