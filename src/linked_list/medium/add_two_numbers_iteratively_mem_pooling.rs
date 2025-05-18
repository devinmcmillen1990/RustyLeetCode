use std::cell::RefCell;

use crate::structs::list_node::ListNode;

// TODO: Try AlignedListNode?

#[derive(Debug)]
struct NodePool {
    pool: RefCell<Vec<Box<ListNode>>>,
}

impl NodePool {
    pub fn new(initial_capacity: usize) -> Self {
        Self {
            pool: RefCell::new(Vec::with_capacity(initial_capacity)),
        }
    }

    fn recycle(&self, node: Box<ListNode>) {
        let mut pool = self.pool.borrow_mut();
        pool.push(node);
    }

    fn get_node(&self, val: i32) -> Box<ListNode> {
        if let Some(node) = self.pool.borrow_mut().pop() {
            let mut reused_node = *node;
            reused_node.val = val;
            reused_node.next = None;
            Box::new(reused_node)
        } else {
            Box::new(ListNode::new(val))
        }
    }
}

const NODE_POOL_SIZE: usize = 10;

pub fn add_two_numbers_iteratively_mem_pooling(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let pool = NodePool::new(NODE_POOL_SIZE);
    add_two_numbers_iteratively_with_pool(l1, l2, &pool)
}

fn add_two_numbers_iteratively_with_pool(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    pool: &NodePool,
) -> Option<Box<ListNode>> {
    let mut result_head = pool.get_node(0);
    let mut current = &mut result_head;
    let mut l1_ = l1;
    let mut l2_ = l2;
    let mut carry = 0;

    while l1_.is_some() || l2_.is_some() || carry != 0 {
        let x = l1_.as_ref().map_or(0, |node| node.val);
        let y = l2_.as_ref().map_or(0, |node| node.val);
        let sum = x + y + carry;

        carry = sum / 10;
        current.next = Some(pool.get_node(sum % 10));
        current = current.next.as_mut().unwrap();

        l1_ = l1_.and_then(|mut node| {
            let next = node.next.take();
            pool.recycle(node);
            next
        });
        l2_ = l2_.and_then(|mut node| {
            let next = node.next.take();
            pool.recycle(node);
            next
        });
    }

    result_head.next
}
