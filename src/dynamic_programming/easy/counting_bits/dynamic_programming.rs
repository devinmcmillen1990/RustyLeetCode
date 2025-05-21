// DP with shift & mask	    O(n)	    O(n)	{dp[i] = dp[i >> 1] + (i & 1)}
pub fn count_bits(n: i32) -> Vec<i32> {
    let mut result = vec![0; (n + 1) as usize];

    for i in 1..=n as usize {
        result[i] = result[i >> 1] + (i & 1) as i32;
    }

    result
}
