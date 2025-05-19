use std::mem::swap;

/*
Median of 2 Sorted Arrays - Optimized Binary Search (O(log(min(m, n))))
Approach:           * The optimal solution leverages the properties of sorted arrays and binary search.
                    * The goal is to partition the two arrays such that the left and right partitions are balanced.
Key Insight:        * We use binary search on the smaller array to minimize the number of elements scanned

Time Complexity:    O(log(min(m, n))) — Binary search on the smaller array.
Space Complexity:   O(1) — Constant space.
*/
pub fn median_of_two_sorted_arrays_binary_search(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums1_ = nums1;
    let mut nums2_ = nums2;

    if nums1_.len() > nums2_.len() {
        swap(&mut nums1_, &mut nums2_);
    }

    let nums1_len = nums1_.len();
    let nums2_len = nums2_.len();
    let half_len = (nums1_len + nums2_len + 1) / 2;
    let mut main_left = 0;
    let mut main_right = nums1_len;

    while main_left <= main_right {
        let i = (main_left + main_right) / 2;
        let j = half_len - i;

        let nums1_left = if i == 0 { i32::MIN } else { nums1_[i - 1] };
        let nums1_right = if i == nums1_len { i32::MAX } else { nums1_[i] };
        let nums2_left = if j == 0 { i32::MIN } else { nums2_[j - 1] };
        let nums2_right = if j == nums2_len { i32::MAX } else { nums2_[j] };

        if nums1_left <= nums2_right && nums2_left <= nums1_right {
            if (nums1_len + nums2_len) % 2 == 0 {
                return (nums1_left.max(nums2_left) as f64 + nums1_right.min(nums2_right) as f64) / 2.0;
            } else {
                return nums1_left.max(nums2_left) as f64;
            }
        } else if nums1_left > nums2_right {
            main_right = i - 1;
        } else {
            main_left = i + 1;
        }
    }

    panic!("Input arrays are not sorted");
}
