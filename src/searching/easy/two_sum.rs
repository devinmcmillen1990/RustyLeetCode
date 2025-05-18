use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut checked = HashMap::<i32, i32>::new();

    for (i, num) in nums.into_iter().enumerate() {
        let desired = target - num;

        if let Some(key) = checked.get(&desired) {
            return vec![i as i32, *key];
        }

        checked.insert(num, i as i32);
    }

    vec![]
}