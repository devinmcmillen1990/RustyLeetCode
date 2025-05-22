use std::collections::HashSet;

pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = HashSet::new();
    let n = nums.len();
    let mut c = vec![0; n];

    result.insert(nums.clone());

    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                nums.swap(0, i);
            } else {
                nums.swap(c[i], i);
            }
            result.insert(nums.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    result.into_iter().collect()
}
