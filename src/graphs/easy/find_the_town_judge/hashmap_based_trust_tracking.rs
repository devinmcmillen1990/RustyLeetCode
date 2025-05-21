use std::collections::HashMap;

pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut trusted_by = HashMap::new();
    let mut trusts_others = HashMap::new();

    for pair in trust {
        *trusts_others.entry(pair[0]).or_insert(0) += 1;
        *trusted_by.entry(pair[1]).or_insert(0) += 1;
    }

    for i in 1..=n {
        if *trusts_others.get(&i).unwrap_or(&0) == 0 && *trusted_by.get(&i).unwrap_or(&0) == n - 1 {
            return i;
        }
    }

    -1
}
