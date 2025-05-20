use std::{cell::RefCell, rc::Rc};

use crate::{
    binary_search_trees::easy::closest_binary_search_tree_value::{
        greedy_search, greedy_search_recursively,
    },
    structs::tree_node::TreeNode,
};

type Node = Option<Rc<RefCell<TreeNode>>>;

#[test]
fn test_closest_value_greedy_search() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let tree = build_tree_from_vec(case.input);
        let result = greedy_search::closest_binary_search_tree_value(tree, case.target);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: expected {}, got {}",
            i, case.expected, result
        );
    }
}

#[test]
fn test_closest_value_recursively() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let tree = build_tree_from_vec(case.input);
        let result = greedy_search_recursively::closest_binary_search_tree_value(tree, case.target);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: expected {}, got {}",
            i, case.expected, result
        );
    }
}

struct TestCase {
    input: Vec<Option<i32>>, // input tree in level-order
    target: f64,
    expected: i32,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            input: vec![Some(4), Some(2), Some(5), Some(1), Some(3), None, None],
            target: 3.714286,
            expected: 4,
        },
        TestCase {
            input: vec![Some(4), Some(2), Some(5), Some(1), Some(3), None, None],
            target: 2.9,
            expected: 3,
        },
        TestCase {
            input: vec![Some(4), Some(2), Some(5), Some(1), Some(3), None, None],
            target: 1.0,
            expected: 1,
        },
        TestCase {
            input: vec![Some(4), Some(2), Some(5), Some(1), Some(3), None, None],
            target: 5.1,
            expected: 5,
        },
    ]
}

// Helper to build a binary tree from Vec<Option<i32>>
fn build_tree_from_vec(values: Vec<Option<i32>>) -> Node {
    use std::collections::VecDeque;

    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;

    while i < values.len() {
        if let Some(current) = queue.pop_front() {
            // Left child
            if let Some(Some(val)) = values.get(i) {
                let left = Rc::new(RefCell::new(TreeNode::new(*val)));
                current.borrow_mut().left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            i += 1;

            // Right child
            if let Some(Some(val)) = values.get(i) {
                let right = Rc::new(RefCell::new(TreeNode::new(*val)));
                current.borrow_mut().right = Some(Rc::clone(&right));
                queue.push_back(right);
            }
            i += 1;
        }
    }

    Some(root)
}
