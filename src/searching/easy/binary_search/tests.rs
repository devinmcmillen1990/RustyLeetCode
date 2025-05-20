use crate::searching::easy::binary_search::binary_search;

#[test]
fn test_binary_search() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = binary_search::search(case.nums.clone(), case.target);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: nums = {:?}, target = {}, expected {}, got {}",
            i, case.nums, case.target, case.expected, result
        );
    }
}

struct TestCase {
    nums: Vec<i32>,
    target: i32,
    expected: i32,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            nums: vec![-1, 0, 3, 5, 9, 12],
            target: 9,
            expected: 4,
        },
        TestCase {
            nums: vec![-1, 0, 3, 5, 9, 12],
            target: 2,
            expected: -1,
        },
        TestCase {
            nums: vec![1],
            target: 1,
            expected: 0,
        },
        TestCase {
            nums: vec![1],
            target: 2,
            expected: -1,
        },
        TestCase {
            nums: vec![],
            target: 5,
            expected: -1,
        },
        TestCase {
            nums: vec![1, 3, 5, 7, 9],
            target: 1,
            expected: 0,
        },
        TestCase {
            nums: vec![1, 3, 5, 7, 9],
            target: 9,
            expected: 4,
        },
    ]
}
