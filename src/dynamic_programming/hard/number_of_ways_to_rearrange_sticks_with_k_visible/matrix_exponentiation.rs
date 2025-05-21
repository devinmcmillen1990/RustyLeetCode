const MOD: i64 = 1_000_000_007;

pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
    let (n, k) = (n as usize, k as usize);
    if n < k {
        return 0;
    }

    let mut dp = vec![0; k + 1];
    dp[0] = 1;

    for i in 1..=n {
        let mut new_dp = vec![0; k + 1];
        for j in 1..=k {
            new_dp[j] = (dp[j - 1] + (i as i64 - 1) * dp[j]) % MOD;
        }
        dp = new_dp;
    }

    dp[k] as i32
}
