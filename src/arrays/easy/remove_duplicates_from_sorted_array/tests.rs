use crate::arrays::easy::remove_duplicates_from_sorted_array::two_pointer;

#[test]
fn test_remove_duplicates() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let mut input_vec = case.input.clone();
        let result_len = two_pointer::remove_duplicates(&mut input_vec);
        assert_eq!(
            result_len, case.expected_length,
            "Case {} failed: Expected length {}, got {}",
            index, case.expected_length, result_len
        );
        assert_eq!(
            &input_vec[..result_len as usize],
            case.expected_values.as_slice(),
            "Case {} failed: Expected prefix {:?}, got {:?}",
            index,
            case.expected_values,
            &input_vec[..result_len as usize]
        );
    }
}

struct TestCase {
    input: Vec<i32>,
    expected_length: i32,
    expected_values: Vec<i32>,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            input: vec![1, 1, 2],
            expected_length: 2,
            expected_values: vec![1, 2],
        },
        TestCase {
            input: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
            expected_length: 5,
            expected_values: vec![0, 1, 2, 3, 4],
        },
        TestCase {
            input: vec![1],
            expected_length: 1,
            expected_values: vec![1],
        },
        TestCase {
            input: vec![],
            expected_length: 0,
            expected_values: vec![],
        },
        TestCase {
            input: vec![1, 2, 3],
            expected_length: 3,
            expected_values: vec![1, 2, 3],
        },
        TestCase {
            input: vec![1, 1, 1, 1],
            expected_length: 1,
            expected_values: vec![1],
        },
    ]
}
