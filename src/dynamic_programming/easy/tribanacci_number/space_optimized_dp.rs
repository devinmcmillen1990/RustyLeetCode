pub fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }

    let (mut a, mut b, mut c) = (0, 1, 1);
    for _ in 3..=n {
        let next = a + b + c;
        a = b;
        b = c;
        c = next;
    }

    c
}
