pub fn tribonacci(n: i32) -> i32 {
    fn helper(n: usize, memo: &mut Vec<i32>) -> i32 {
        if memo[n] != -1 {
            return memo[n];
        }
        memo[n] = helper(n - 1, memo) + helper(n - 2, memo) + helper(n - 3, memo);
        memo[n]
    }

    let n = n as usize;
    let mut memo = vec![-1; n.max(3) + 1];
    memo[0] = 0;
    memo[1] = 1;
    memo[2] = 1;
    helper(n, &mut memo)
}
