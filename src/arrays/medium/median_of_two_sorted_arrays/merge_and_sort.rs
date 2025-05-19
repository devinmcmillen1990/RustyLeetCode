/*
Median of 2 Sorted Arrays - Merge and Sort (O((m + n) log(m + n)))
Approach:           * Merge both arrays and sort the merged array.
                    * The median is either the middle element (if the length is odd) or the average
                      of the two middle elements (if the length is even).

Time Complexity:    O((m + n) log(m + n)) — Due to sorting.
Space Complexity:   O(m + n) — For the merged array.
*/
pub fn median_of_two_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged_nums = [nums1, nums2].concat();
    merged_nums.sort_unstable();

    let merged_len = merged_nums.len();

    if merged_len % 2 == 0 {
        (merged_nums[merged_len / 2 - 1] as f64 + merged_nums[merged_len / 2] as f64) / 2.0
    } else {
        merged_nums[merged_len / 2] as f64
    }
}
