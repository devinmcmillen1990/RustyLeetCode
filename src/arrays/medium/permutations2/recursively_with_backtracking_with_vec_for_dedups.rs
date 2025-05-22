pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    // sort to detect duplicates
    nums.sort();

    let mut result = vec![];
    let mut path = vec![];
    let mut used = vec![false; nums.len()];
    backtrack(&mut path, &mut used, &nums, &mut result);
    result
}

fn backtrack(
    path: &mut Vec<i32>,
    used: &mut Vec<bool>,
    nums: &Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if path.len() == nums.len() {
        result.push(path.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] {
            continue;
        }

        // Skip duplicate at the same tree depth
        if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
            continue;
        }

        used[i] = true;
        path.push(nums[i]);

        backtrack(path, used, nums, result);

        path.pop();
        used[i] = false;
    }
}
