pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut memo = vec![0; (n + 1) as usize];
    memo[1] = 1;
    fib_with_memoization(n, &mut memo)
}

fn fib_with_memoization(n: i32, memo: &mut Vec<i32>) -> i32 {
    if n == 0 {
        return 0;
    }
    
    let n_index = n as usize;

    if memo[n_index] != 0 {
        return memo[n_index];
    }

    memo[n_index] = fib_with_memoization(n - 1, memo) + fib_with_memoization(n - 2, memo);
    memo[n_index]
}
