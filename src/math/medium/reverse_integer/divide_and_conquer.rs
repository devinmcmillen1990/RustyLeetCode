/*
Reverse Integer - Divide and Conquer Approach (O(log₁₀ n))
Approach:           * Extract the most significant digit and the least significant digit.
                    * Swap their positions iteratively.
                    * Handle overflow by checking bounds after each swap.

Time Complexity:    O(log₁₀ n)
Space Complexity:   O(1)
*/
pub fn reverse_integer(x: i32) -> i32 {
    let mut x = x;
    let mut rev = 0;

    while x != 0 {
        let digit = x % 10;
        x /= 10;

        // Overflow checks
        if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && digit > 7) {
            return 0;
        }
        if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && digit < -8) {
            return 0;
        }

        rev = rev * 10 + digit;
    }

    rev
}
