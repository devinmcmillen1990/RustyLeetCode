// DP with offset (MSB)	    O(n)	    O(n)	    {dp[i] = 1 + dp[i - offset]}
pub fn count_bits(n: i32) -> Vec<i32> {
    let mut result = vec![0; (n + 1) as usize];
    let mut offset = 1;

    for i in 1..=n as usize {
        if offset * 2 == i {
            offset = i;
        }
        result[i] = 1 + result[i - offset];
    }

    result
}
