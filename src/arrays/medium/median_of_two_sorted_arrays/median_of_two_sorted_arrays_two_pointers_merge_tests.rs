use super::{
    median_of_two_sorted_arrays_tests_helpers::{
        assert_median_of_two_sorted_arrays, get_test_cases,
    },
    median_of_two_sorted_arrays_two_pointers_merge::median_of_two_sorted_arrays_two_pointers_merge,
};

#[test]
fn test_median_of_two_sorted_arrays_merged_and_sorted() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = median_of_two_sorted_arrays_two_pointers_merge(case.nums1, case.nums2);
        assert_median_of_two_sorted_arrays(result, case.expected, index);
    }
}
