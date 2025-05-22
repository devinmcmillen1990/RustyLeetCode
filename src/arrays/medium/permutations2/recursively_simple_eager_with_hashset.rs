use std::collections::HashSet;

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![vec![]];
    }

    let mut result = Vec::new();
    let mut used = HashSet::new(); // Dedup *within this level only*

    for (i, &num) in nums.iter().enumerate() {
        if used.contains(&num) {
            continue;
        }
        used.insert(num);

        let mut remaining = nums.clone();
        remaining.remove(i);

        for mut perm in permute_unique(remaining) {
            let mut new_perm = vec![num];
            new_perm.append(&mut perm);
            result.push(new_perm);
        }
    }

    result
}
