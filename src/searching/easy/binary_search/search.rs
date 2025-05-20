pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    search_recursively(nums.clone(), target, 0, nums.len())
}

#[inline(always)]
fn search_recursively(nums: Vec<i32>, target: i32, start: usize, end: usize) -> i32 {
    let mid: usize = (end + start) / 2;

    if mid >= end {
        return -1;
    }

    if nums[mid] == target {
        return mid as i32;
    }

    if target > nums[mid] {
        search_recursively(nums, target, mid + 1, end)
    } else {
        search_recursively(nums, target, start, mid)
    }
}
