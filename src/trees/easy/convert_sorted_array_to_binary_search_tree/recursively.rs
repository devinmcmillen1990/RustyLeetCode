use std::{cell::RefCell, rc::Rc};

use crate::structs::tree_node::TreeNode;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    sorted_array_to_bst_recursively(&nums)
}

fn sorted_array_to_bst_recursively(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let mid = nums.len() / 2;
    let node = Rc::new(RefCell::new(TreeNode::new(nums[mid])));

    node.borrow_mut().left = sorted_array_to_bst_recursively(&nums[..mid]);
    node.borrow_mut().right = sorted_array_to_bst_recursively(&nums[mid + 1..]);
    Some(node)
}