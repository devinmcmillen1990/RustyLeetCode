use crate::linked_list::medium::add_two_numbers::{
    add_two_nums_iteratively, add_two_nums_recursively,
};
use crate::structs::list_node::ListNode;

#[cfg(test)]
struct TestCase {
    l1: Vec<i32>,
    l2: Vec<i32>,
    expected: Vec<i32>,
}

fn get_test_cases() -> Vec<TestCase> {
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

/// Helper function to convert a vector to a linked list.
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut current = &mut dummy_head;

    for &val in &vec {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }

    dummy_head.next
}

/// Helper function to convert a linked list to a vector.
fn list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(n) = node {
        result.push(n.val);
        node = n.next;
    }
    result
}

/// Custom assertion function to verify output.
fn assert_linked_list_eq(actual: Option<Box<ListNode>>, expected: Vec<i32>, case_index: usize) {
    let actual_vec = list_to_vec(actual);
    assert_eq!(
        actual_vec, expected,
        "Test case {} failed: Expected {:?}, got {:?}",
        case_index, expected, actual_vec
    );
}

#[test]
fn test_add_two_nums_iteratively() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let l1 = vec_to_list(case.l1);
        let l2 = vec_to_list(case.l2);
        let result = add_two_nums_iteratively(l1, l2);
        assert_linked_list_eq(result, case.expected, index);
    }
}

#[test]
fn test_add_two_nums_recursive() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let l1 = vec_to_list(case.l1);
        let l2 = vec_to_list(case.l2);
        let result = add_two_nums_recursively(l1, l2);
        assert_linked_list_eq(result, case.expected, index);
    }
}
