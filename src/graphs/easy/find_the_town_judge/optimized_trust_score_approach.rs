pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    // TODO: This is dangerous. Setup parameter for the length of this array.
    let mut score = [0i32; 100001]; // avoids heap

    for pair in trust {
        let a = pair[0] as usize;
        let b = pair[1] as usize;
        score[a] -= 1;
        score[b] += 1;
    }

    for i in 1..=n as usize {
        if score[i] == n - 1 {
            return i as i32;
        }
    }

    -1
}
