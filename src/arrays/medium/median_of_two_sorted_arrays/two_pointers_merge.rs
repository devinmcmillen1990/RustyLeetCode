/*
Median of 2 Sorted Arrays - Merge and Sort (O((m + n) log(m + n)))
Approach:           * Merge both arrays and sort the merged array.
                    * The median is either the middle element (if the length is odd) or the average
                      of the two middle elements (if the length is even).

Time Complexity:    O(m + n) — Linear scan through both arrays.
Space Complexity:   O(m + n) — For the merged array.
*/
pub fn median_of_two_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let nums1_len = nums1.len();
    let nums2_len = nums2.len();

    let mut merged_nums = Vec::with_capacity(nums1_len + nums2_len);

    let mut i = 0;
    let mut j = 0;

    while i < nums1_len && j < nums2_len {
        if nums1[i] < nums2[j] {
            merged_nums.push(nums1[i]);
            i += 1;
        } else {
            merged_nums.push(nums2[j]);
            j += 1;
        }
    }

    merged_nums.extend_from_slice(&nums1[i..]);
    merged_nums.extend_from_slice(&nums2[j..]);

    let merged_len = merged_nums.len();
    if merged_len % 2 == 0 {
        (merged_nums[merged_len / 2 - 1] as f64 + merged_nums[merged_len / 2] as f64) / 2.0
    } else {
        merged_nums[merged_len / 2] as f64
    }
}
