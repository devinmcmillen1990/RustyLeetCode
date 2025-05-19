/*
Reverse Integer - Mathematical Approach (O(log₁₀ n))
Approach:           * Extract the digits of the number one by one using the modulus operator %.
                    * Construct the reversed number by appending the extracted digits to a new number.
                    * Handle overflow by checking the reversed number against the bounds of a 32-bit signed integer.

Time Complexity:    O(log₁₀ n) — Each iteration processes one digit.
Space Complexity:   O(1) — Constant space usage.
*/
pub fn reverse_integer(x: i32) -> i32 {
    let mut x = x;
    let mut rev = 0;

    while x != 0 {
        let digit = x % 10;
        x /= 10;

        // Check for overflow before multiplying
        if (rev > i32::MAX / 10) || (rev == i32::MAX / 10 && digit > 7) {
            return 0;
        }

        if (rev < i32::MIN / 10) || (rev == i32::MIN / 10 && digit < -8) {
            return 0;
        }

        rev = rev * 10 + digit;
    }

    rev
}
