pub fn fib(n: i32) -> i32 {
    if n <= 0 { return 0; }
    if n == 1 { return 1; }
    return fib(n-2) + fib(n-1);
}