pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let (m, n) = (s1.len(), s2.len());
    if m + n != s3.len() {
        return false;
    }

    let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    for i in 0..=m {
        for j in 0..=n {
            if i > 0 && s1[i - 1] == s3[i + j - 1] {
                dp[i][j] |= dp[i - 1][j];
            }
            if j > 0 && s2[j - 1] == s3[i + j - 1] {
                dp[i][j] |= dp[i][j - 1];
            }
        }
    }

    dp[m][n]
}
