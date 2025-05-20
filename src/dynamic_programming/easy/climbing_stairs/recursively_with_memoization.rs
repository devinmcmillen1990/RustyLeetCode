pub fn climb_stairs(n: i32) -> i32 {
    let mut memo = vec![-1; (n + 1) as usize];
    climb(n as usize, &mut memo)
}

fn climb(n: usize, memo: &mut Vec<i32>) -> i32 {
    if n <= 1 {
        return 1;
    }

    if memo[n] != -1 {
        return memo[n];
    }

    memo[n] = climb(n - 1, memo) + climb(n - 2, memo);
    memo[n]
}
