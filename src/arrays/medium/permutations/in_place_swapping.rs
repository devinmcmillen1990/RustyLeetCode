pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(start: usize, nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            result.push(nums.clone());
            return;
        }

        for i in start..nums.len() {
            nums.swap(start, i);
            backtrack(start + 1, nums, result);
            nums.swap(start, i); // backtrack
        }
    }

    let mut nums = nums;
    let mut result = Vec::new();
    backtrack(0, &mut nums, &mut result);
    result
}
