use crate::searching::easy::two_sum::two_sum;

#[test]
fn test_two_sum() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let result = two_sum(case.nums.clone(), case.target);
        assert_unordered_eq(result, case.expected, index);
    }
}

#[cfg(test)]
struct TestCase {
    nums: Vec<i32>,
    target: i32,
    expected: Vec<i32>,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            nums: vec![2, 7, 11, 15],
            target: 9,
            expected: vec![0, 1],
        },
        TestCase {
            nums: vec![3, 2, 4],
            target: 6,
            expected: vec![1, 2],
        },
        TestCase {
            nums: vec![3, 3],
            target: 6,
            expected: vec![0, 1],
        },
        TestCase {
            nums: vec![1, 2, 3, 4, 5],
            target: 8,
            expected: vec![2, 4],
        },
        TestCase {
            nums: vec![0, 4, 3, 0],
            target: 0,
            expected: vec![0, 3],
        },
        TestCase {
            nums: vec![-1, -2, -3, -4, -5],
            target: -8,
            expected: vec![2, 4],
        },
    ]
}

fn assert_unordered_eq(mut actual: Vec<i32>, mut expected: Vec<i32>, case_index: usize) {
    actual.sort_unstable();
    expected.sort_unstable();
    assert_eq!(
        actual, expected,
        "Test case {} failed: Expected {:?}, got {:?}",
        case_index, expected, actual
    );
}
