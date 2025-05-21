pub fn can_win_nim(n: i32) -> bool {
    if n <= 3 {
        return true;
    }

    let mut dp = vec![false; (n + 1) as usize];
    dp[1] = true;
    dp[2] = true;
    dp[3] = true;

    for i in 4..=n {
        dp[i as usize] = !dp[(i - 1) as usize] || !dp[(i - 2) as usize] || !dp[(i - 3) as usize];
    }

    dp[n as usize]
}
