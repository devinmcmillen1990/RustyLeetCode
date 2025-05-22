use std::collections::HashSet;

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums_ = nums;
    let mut set = HashSet::new();
    backtrack(0, &mut nums_, &mut set);
    set.into_iter().collect()
}

fn backtrack(start: usize, nums: &mut Vec<i32>, set: &mut HashSet<Vec<i32>>) {
    if start == nums.len() {
        set.insert(nums.clone());
        return;
    }

    for i in start..nums.len() {
        nums.swap(start, i);
        backtrack(start + 1, nums, set);
        nums.swap(start, i);
    }
}
