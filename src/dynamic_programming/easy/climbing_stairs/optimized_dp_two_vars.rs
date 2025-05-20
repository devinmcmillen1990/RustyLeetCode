pub fn climb_stairs(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    let (mut prev, mut curr) = (1, 1);

    for _ in 2..=n {
        let tmp = curr;
        curr = curr + prev;
        prev = tmp;
    }

    curr
}
