pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
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

            used[i] = true;
            path.push(nums[i]);
            backtrack(path, used, nums, result);
            path.pop();
            used[i] = false;
        }
    }

    let mut result = Vec::new();
    let mut path = Vec::new();
    let mut used = vec![false; nums.len()];
    backtrack(&mut path, &mut used, &nums, &mut result);
    result
}
