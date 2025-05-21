use crate::structs::tree_node::TreeNode;
use std::{cell::RefCell, rc::Rc};

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn closest_binary_search_tree_value(root: Node, target: f64) -> i32 {
    if let Some(rc_node) = &root {
        closest_binary_search_tree_value_recursively(&root, target, rc_node.borrow().val)
    } else {
        panic!("Tree is empty");
    }
}

fn closest_binary_search_tree_value_recursively(node: &Node, target: f64, closest: i32) -> i32 {
    if let Some(rc_node) = node {
        let node_ref = rc_node.borrow();
        let val = node_ref.val;
        let new_closest = if (target - val as f64).abs() < (target - closest as f64).abs() {
            val
        } else {
            closest
        };

        if target < val as f64 {
            closest_binary_search_tree_value_recursively(&node_ref.left, target, new_closest)
        } else if target > val as f64 {
            closest_binary_search_tree_value_recursively(&node_ref.right, target, new_closest)
        } else {
            val // exact match
        }
    } else {
        closest
    }
}
