/*
Reverse Integer - Recursive Approach (O(log₁₀ n))
Approach:           * Similar to the mathematical approach, but implemented recursively.
                    * Extract the last digit and call the function recursively on the remaining number.

Time Complexity:    O(log₁₀ n)
Space Complexity:   O(log₁₀ n) — Due to recursion stack.
*/
pub fn reverse_integer(x: i32) -> i32 {
    fn helper(x: i32, rev: i32) -> i32 {
        if x == 0 {
            return rev;
        }

        let digit = x % 10;
        let new_rev = rev.checked_mul(10).and_then(|r| r.checked_add(digit));

        match new_rev {
            Some(val) => helper(x / 10, val),
            None => 0,
        }
    }

    helper(x, 0)
}
