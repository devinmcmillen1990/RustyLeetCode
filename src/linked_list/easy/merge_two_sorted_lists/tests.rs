use crate::linked_list::easy::merge_two_sorted_lists::{recursively, iteratively};
use crate::structs::list_node::ListNode;

#[test]
fn test_merge_two_lists_two_pointers() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let list1 = to_linked_list(case.list1);
        let list2 = to_linked_list(case.list2);
        let expected = case.expected;
        let result = iteratively::merge_two_lists(list1, list2);
        let result_vec = from_linked_list(result);

        assert_eq!(
            result_vec, expected,
            "Test case {} failed: Expected {:?}, got {:?}",
            index, expected, result_vec
        );
    }
}

#[test]
fn test_merge_two_lists_recursively() {
    for (index, case) in get_test_cases().into_iter().enumerate() {
        let list1 = to_linked_list(case.list1);
        let list2 = to_linked_list(case.list2);
        let expected = case.expected;
        let result = recursively::merge_two_lists(list1, list2);
        let result_vec = from_linked_list(result);

        assert_eq!(
            result_vec, expected,
            "Test case {} failed: Expected {:?}, got {:?}",
            index, expected, result_vec
        );
    }
}

#[cfg(test)]
fn to_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    use crate::structs::list_node::ListNode;

    let mut current = None;
    for &val in values.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = current;
        current = Some(node);
    }
    current
}

#[cfg(test)]
pub fn from_linked_list(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;
    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }
    result
}

#[cfg(test)]
struct TestCase {
    list1: Vec<i32>,
    list2: Vec<i32>,
    expected: Vec<i32>,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            list1: vec![1, 2, 4],
            list2: vec![1, 3, 4],
            expected: vec![1, 1, 2, 3, 4, 4],
        },
        TestCase {
            list1: vec![],
            list2: vec![],
            expected: vec![],
        },
        TestCase {
            list1: vec![],
            list2: vec![0],
            expected: vec![0],
        },
        TestCase {
            list1: vec![5, 10],
            list2: vec![2, 3, 11],
            expected: vec![2, 3, 5, 10, 11],
        },
        TestCase {
            list1: vec![1, 3, 5],
            list2: vec![2, 4, 6],
            expected: vec![1, 2, 3, 4, 5, 6],
        },
    ]
}
