use std::{cell::RefCell, rc::Rc};

use crate::{
    trees::easy::convert_sorted_array_to_binary_search_tree::{
        iteratively, recursively,
    },
    structs::tree_node::TreeNode,
};

#[test]
fn test_sorted_array_to_bst_recursively() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let tree = recursively::sorted_array_to_bst(case.input.clone());
        let in_order = in_order_traversal(tree);
        assert_eq!(
            in_order, case.input,
            "Test case {} failed: expected in-order {:?}, got {:?}",
            i, case.input, in_order
        );
    }
}

#[test]
fn test_sorted_array_to_bst_iteratively() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let tree = iteratively::sorted_array_to_bst(case.input.clone());
        let in_order = in_order_traversal(tree);
        assert_eq!(
            in_order, case.input,
            "Test case {} failed: expected in-order {:?}, got {:?}",
            i, case.input, in_order
        );
    }
}

struct TestCase {
    input: Vec<i32>,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            input: vec![-10, -3, 0, 5, 9],
        },
        TestCase {
            input: vec![1, 2, 3],
        },
        TestCase { input: vec![] },
        TestCase { input: vec![0] },
    ]
}

fn in_order_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn helper(node: Option<Rc<RefCell<TreeNode>>>, output: &mut Vec<i32>) {
        if let Some(rc_node) = node {
            let node_ref = rc_node.borrow();
            helper(node_ref.left.clone(), output);
            output.push(node_ref.val);
            helper(node_ref.right.clone(), output);
        }
    }

    let mut result = Vec::new();
    helper(root, &mut result);
    result
}
