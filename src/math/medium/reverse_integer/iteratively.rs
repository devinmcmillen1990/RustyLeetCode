/*
Reverse Integer - Mathematical Approach (O(log₁₀ n))
Approach:           * Extract the digits of the number one by one using the modulus operator %.
                    * Construct the reversed number by appending the extracted digits to a new number.
                    * Handle overflow by checking the reversed number against the bounds of a 32-bit signed integer.

Time Complexity:    O(log₁₀ n) — Each iteration processes one digit.
Space Complexity:   O(1) — Constant space usage.

TODO: Optimizations
1. Optimized Mathematical Approach (O(log₁₀ n)):
    - We can minimize the number of operations by consolidating checks and using fewer conditional statements.
    -Instead of calculating the overflow conditions twice, we can combine them.
2. Bit Manipulation Approach
    - This approach is less intuitive but can leverage bitwise operations to reverse digits more efficiently.
    - However, since the problem deals with decimal digits, bit manipulation is less effective without significant 
      restructuring.
3. SIMD Optimization (Not for these purposes)
    - If we were dealing with larger data sets or multiple numbers to reverse, we could leverage SIMD 
      (std::simd or packed_simd crate).
    - For a single integer, SIMD doesn't provide significant benefits, but it's an avenue for exploration if 
      batch processing is required.
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
