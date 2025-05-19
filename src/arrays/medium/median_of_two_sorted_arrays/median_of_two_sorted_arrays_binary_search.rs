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
    let mut a = nums1;
    let mut b = nums2;

    if a.len() > b.len() {
        swap(&mut a, &mut b);
    }

    let nums1_len = a.len();
    let nums2_len = b.len();
    let half_len = (nums1_len + nums2_len + 1) / 2;
    let mut left = 0;
    let mut right = nums1_len;

    while left <= right {
        let i = (left + right) / 2;
        let j = half_len - i;

        let a_left = if i == 0 { i32::MIN } else { a[i - 1] };
        let a_right = if i == nums1_len { i32::MAX } else { a[i] };
        let b_left = if j == 0 { i32::MIN } else { b[j - 1] };
        let b_right = if j == nums2_len { i32::MAX } else { b[j] };

        if a_left <= b_right && b_left <= a_right {
            if (nums1_len + nums2_len) % 2 == 0 {
                return (a_left.max(b_left) as f64 + a_right.min(b_right) as f64) / 2.0;
            } else {
                return a_left.max(b_left) as f64;
            }
        } else if a_left > b_right {
            right = i - 1;
        } else {
            left = i + 1;
        }
    }

    panic!("Input arrays are not sorted");
}
