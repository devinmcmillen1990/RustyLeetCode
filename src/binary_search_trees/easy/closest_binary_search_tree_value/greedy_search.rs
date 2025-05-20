use std::{cell::RefCell, rc::Rc};
use crate::structs::tree_node::TreeNode;

/*
Given a non-empty binary search tree and a target value, find the value in the BST that is closest to the target.

Note:
    - Given target value is a floating point.
    - You are guaranteed to have only one unique value in the BST that is closest to the target.

Example:

Input: root = [4,2,5,1,3], target = 3.714286

    4
   / \
  2   5
 / \
1   3

Output: 4
*/

/*
Strategy (Optimal): BST Property + Greedy Search
    - Since it’s a BST:
        - Go left if target < node.val
        - Go right if target > node.val
    - Track the closest value as you go.
    - Time: O(h), where h is the height of the tree.
*/
type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn closest_binary_search_tree_value(root: Node, target: f64) -> i32 {
    let mut node = root;
    let mut closest = i32::MAX;

    while let Some(current) = node {
        let current_val = current.borrow().val;
        if (target - current_val as f64).abs() < (target - closest as f64).abs() {
            closest = current_val;
        }

        node = if target < current_val as f64 {
            current.borrow().left.clone()
        } else {
            current.borrow().right.clone()
        };
    }

    closest
}
