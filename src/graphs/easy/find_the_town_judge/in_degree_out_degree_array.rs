// In-Degree / Out-Degree Array implementation
pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    if n == 1 && trust.is_empty() {
        return 1; // The only person is the judge
    }

    let n = n as usize;
    let mut in_degree = vec![0; n + 1];
    let mut out_degree = vec![0; n + 1];

    for pair in trust {
        let a = pair[0] as usize;
        let b = pair[1] as usize;

        out_degree[a] += 1;
        in_degree[b] += 1;
    }

    for i in 1..=n {
        if in_degree[i] == n - 1 && out_degree[i] == 0 {
            return i as i32;
        }
    }

    -1
}
