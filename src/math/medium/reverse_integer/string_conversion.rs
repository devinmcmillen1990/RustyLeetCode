/*
Reverse Integer - String Conversion Approach (O(n))
Approach:           * Convert the integer to a string.
                    * Reverse the string and handle the sign separately.
                    * Check for overflow after converting the string back to an integer.

Time Complexity:    O(n) — String reversal is linear.
Space Complexity:   O(n) — Extra space for the string.
*/
#[inline(always)]
pub fn reverse_integer(x: i32) -> i32 {
    if x == i32::MIN {
        return 0; // Explicitly handle -2147483648 as it overflows on abs()
    }

    let is_negative = x < 0;
    let s = x.abs().to_string();
    let reversed: String = s.chars().rev().collect();

    match reversed.parse::<i32>() {
        Ok(num) => {
            if is_negative {
                num.checked_neg().unwrap_or(0) // Safely handle the negation
            } else {
                num
            }
        }
        Err(_) => 0,
    }
}
