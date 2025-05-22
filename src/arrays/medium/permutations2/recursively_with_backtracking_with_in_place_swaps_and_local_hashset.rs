use std::collections::HashSet;

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums_ = nums;
    let mut result = Vec::new();
    backtrack(0, &mut nums_, &mut result);
    result
}

fn backtrack(start: usize, nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if start == nums.len() {
        result.push(nums.clone());
        return;
    }

    let mut seen = HashSet::new();

    for i in start..nums.len() {
        // Skip duplicates
        if seen.contains(&nums[i]) {
            continue;
        }

        seen.insert(nums[i]);

        nums.swap(start, i);
        backtrack(start + 1, nums, result);
        nums.swap(start, i);
    }
}
