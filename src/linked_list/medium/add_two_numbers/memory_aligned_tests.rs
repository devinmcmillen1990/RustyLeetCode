use super::memory_aligned::add_two_numbers;
use crate::structs::list_node::AlignedListNode;

#[test]
fn test_add_two_nums_iteratively() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let l1 = vec_to_aligned_list(case.l1);
        let l2 = vec_to_aligned_list(case.l2);
        let result = add_two_numbers(l1, l2);
        assert_aligned_linked_list_eq(result, case.expected, index);
    }
}

#[cfg(test)]
pub struct TestCase {
    pub l1: Vec<i32>,
    pub l2: Vec<i32>,
    pub expected: Vec<i32>,
}

#[cfg(test)]
pub fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            l1: vec![2, 4, 3],
            l2: vec![5, 6, 4],
            expected: vec![7, 0, 8],
        },
        TestCase {
            l1: vec![0],
            l2: vec![0],
            expected: vec![0],
        },
        TestCase {
            l1: vec![9, 9, 9, 9, 9, 9, 9],
            l2: vec![9, 9, 9, 9],
            expected: vec![8, 9, 9, 9, 0, 0, 0, 1],
        },
        TestCase {
            l1: vec![1],
            l2: vec![9],
            expected: vec![0, 1],
        },
        TestCase {
            l1: vec![5],
            l2: vec![5],
            expected: vec![0, 1],
        },
        TestCase {
            l1: vec![9, 9],
            l2: vec![1],
            expected: vec![0, 0, 1],
        },
    ]
}

/// Helper function to convert a vector to a (AlignedListNode) linked list.
#[cfg(test)]
pub fn vec_to_aligned_list(vec: Vec<i32>) -> Option<Box<AlignedListNode>> {
    let mut dummy_head = Box::new(AlignedListNode::new(0));
    let mut current = &mut dummy_head;

    for &val in &vec {
        current.next = Some(Box::new(AlignedListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }

    dummy_head.next
}

/// Helper function to convert a (AlignedListNode) linked list to a vector.
fn list_to_vec(mut node: Option<Box<AlignedListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(n) = node {
        result.push(n.val);
        node = n.next;
    }
    result
}

/// Custom assertion function to verify output.
#[cfg(test)]
pub fn assert_aligned_linked_list_eq(
    actual: Option<Box<AlignedListNode>>,
    expected: Vec<i32>,
    case_index: usize,
) {
    let actual_vec = list_to_vec(actual);
    assert_eq!(
        actual_vec, expected,
        "Test case {} failed: Expected {:?}, got {:?}",
        case_index, expected, actual_vec
    );
}
