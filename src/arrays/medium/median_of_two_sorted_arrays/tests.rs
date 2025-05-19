use super::{
    binary_search,
    merge_and_sort,
    two_pointers_merge,
};

#[test]
fn test_median_of_two_sorted_arrays_merge() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = binary_search::median_of_two_sorted_arrays(case.nums1, case.nums2);
        assert_median_of_two_sorted_arrays(result, case.expected, index);
    }
}

#[test]
fn test_median_of_two_sorted_arrays_merged_and_sorted() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = merge_and_sort::median_of_two_sorted_arrays(case.nums1, case.nums2);
        assert_median_of_two_sorted_arrays(result, case.expected, index);
    }
}

#[test]
fn test_median_of_two_sorted_arrays_two_pointers_merge() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = two_pointers_merge::median_of_two_sorted_arrays(case.nums1, case.nums2);
        assert_median_of_two_sorted_arrays(result, case.expected, index);
    }
}

#[cfg(test)]
pub struct TestCase {
    pub nums1: Vec<i32>,
    pub nums2: Vec<i32>,
    pub expected: f64,
}

#[cfg(test)]
pub fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            nums1: vec![1, 3],
            nums2: vec![2],
            expected: 2.0, // [1, 2, 3] - Median is 2
        },
        TestCase {
            nums1: vec![1, 2],
            nums2: vec![3, 4],
            expected: 2.5, // [1, 2, 3, 4] - Median is (2 + 3) / 2
        },
        TestCase {
            nums1: vec![0, 0],
            nums2: vec![0, 0],
            expected: 0.0, // [0, 0, 0, 0] - Median is 0
        },
        TestCase {
            nums1: vec![],
            nums2: vec![1],
            expected: 1.0, // [1] - Single element
        },
        TestCase {
            nums1: vec![2],
            nums2: vec![],
            expected: 2.0, // [2] - Single element
        },
        TestCase {
            nums1: vec![1, 2],
            nums2: vec![1, 2, 3],
            expected: 2.0, // [1, 1, 2, 2, 3] - Median is 2
        },
        TestCase {
            nums1: vec![1, 3],
            nums2: vec![2, 7],
            expected: 2.5, // [1, 2, 3, 7] - Median is (2 + 3) / 2
        },
        TestCase {
            nums1: vec![3],
            nums2: vec![1, 2, 4],
            expected: 2.5, // [1, 2, 3, 4] - Median is (2 + 3) / 2
        },
        TestCase {
            nums1: vec![1, 2, 6, 8],
            nums2: vec![3, 4, 5, 7],
            expected: 4.5, // [1, 2, 3, 4, 5, 6, 7, 8] - Median is (4 + 5) / 2
        },
        TestCase {
            nums1: vec![],
            nums2: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            expected: 5.0, // Median is 5
        },
    ]
}

/// Custom assertion function to verify output.
#[cfg(test)]
pub fn assert_median_of_two_sorted_arrays(actual: f64, expected: f64, case_index: usize) {
    assert!(
        (actual - expected).abs() < 1e-6,
        "Test case {} failed: Expected {}, got {}",
        case_index,
        expected,
        actual
    );
}
