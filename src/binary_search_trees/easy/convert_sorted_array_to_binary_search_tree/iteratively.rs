use std::{cell::RefCell, rc::Rc};

use crate::structs::tree_node::TreeNode;

struct Frame {
    start: usize,
    end: usize,
    parent: Option<Rc<RefCell<TreeNode>>>,
    is_left: bool,
}

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Node {
    if nums.is_empty() {
        return None;
    }

    let mid = (nums.len() - 1) / 2;
    let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));

    let mut statck = vec![
        Frame {
            start: 0,
            end: mid,
            parent: Some(Rc::clone(&root)),
            is_left: true,
        },
        Frame {
            start: mid + 1,
            end: nums.len(),
            parent: Some(Rc::clone(&root)),
            is_left: false,
        },
    ];

    while let Some(Frame {
        start,
        end,
        parent,
        is_left,
    }) = statck.pop()
    {
        if start >= end {
            continue;
        }

        let mid = (start + end - 1) / 2;
        let node = Rc::new(RefCell::new(TreeNode::new(nums[mid])));

        // Attach to parent
        if let Some(p) = parent {
            if is_left {
                p.borrow_mut().left = Some(Rc::clone(&node));
            } else {
                p.borrow_mut().right = Some(Rc::clone(&node));
            }
        }

        // Push right and left children
        statck.push(Frame {
            start: mid + 1,
            end,
            parent: Some(Rc::clone(&node)),
            is_left: false,
        });
        statck.push(Frame {
            start,
            end: mid,
            parent: Some(Rc::clone(&node)),
            is_left: true,
        });
    }
    Some(root)
}
