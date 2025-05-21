pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let k = k as usize;

    let mut memo = vec![vec![-1; k + 2]; n + 2];

    fn dp(n: usize, k: usize, memo: &mut Vec<Vec<i64>>) -> i64 {
        if n == 0 && k == 0 {
            return 1;
        }
        if n == 0 || k == 0 {
            return 0;
        }
        if memo[n][k] != -1 {
            return memo[n][k];
        }

        let add = (dp(n - 1, k - 1, memo) + (n as i64 - 1) * dp(n - 1, k, memo)) % MOD;
        memo[n][k] = add;
        add
    }

    dp(n, k, &mut memo) as i32
}
