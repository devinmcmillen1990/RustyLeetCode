pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if nums.is_empty() {
        result.push(Vec::new()); // Base case: empty input, return a vec containing an empty vec
        return result;
    }

    for (i, item) in nums.iter().enumerate() {
        let mut remaining = nums.clone();
        remaining.remove(i);

        let sub_permutations = permute(remaining);

        for sub_perm in sub_permutations {
            let mut perm = Vec::new();
            perm.push(*item);
            perm.extend(sub_perm);
            result.push(perm);
        }
    }
    result
}
