pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut trust_score = vec![0; (n + 1) as usize];

    for pair in trust {
        let a = pair[0] as usize;
        let b = pair[1] as usize;
        trust_score[a] -= 1;
        trust_score[b] += 1;
    }

    for i in 1..=n as usize {
        if trust_score[i] == (n - 1) {
            return i as i32;
        }
    }

    -1
}
