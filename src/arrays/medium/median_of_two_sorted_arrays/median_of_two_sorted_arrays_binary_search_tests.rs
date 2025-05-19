use super::{
    median_of_two_sorted_arrays_binary_search::median_of_two_sorted_arrays_binary_search,
    median_of_two_sorted_arrays_tests_helpers::{
        assert_median_of_two_sorted_arrays, get_test_cases,
    },
};

#[test]
fn test_median_of_two_sorted_arrays_merge() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = median_of_two_sorted_arrays_binary_search(case.nums1, case.nums2);
        assert_median_of_two_sorted_arrays(result, case.expected, index);
    }
}
