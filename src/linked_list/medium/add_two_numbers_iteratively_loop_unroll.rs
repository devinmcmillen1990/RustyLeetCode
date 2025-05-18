/*
Unrolling the Loop:
    We can manually unroll the iteration loop to handle multiple nodes in a single iteration. However, this may not be
    as effective due to the nature of linked list traversal being pointer-based.
*/

// TODO: Get Time/Space Complexity Update

use crate::structs::list_node::ListNode;

pub fn add_two_numbers_iteratively_loop_unroll(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result_head = Box::new(ListNode::new(0));
    let mut current = &mut result_head;
    let mut carry = 0;
    let mut l1_ = l1;
    let mut l2_ = l2;

    while l1_.is_some() || l2_.is_some() || carry != 0 {
        let mut sums = [0, 0, 0, 0];
        let mut processed = 0;

        // Unroll the original loop by processing the digits in 4-decimal-place increments.
        for i in 0..4 {
            if l1_.is_some() || l2_.is_some() || carry != 0 {
                processed += 1;

                let mut sum = carry;

                if let Some(node) = l1_.take() {
                    sum += node.val;
                    l1_ = node.next;
                }

                if let Some(node) = l2_.take() {
                    sum += node.val;
                    l2_ = node.next;
                }

                carry = sum / 10;
                sums[i] = sum % 10;
            }
        }

        // Apply the calculated nodes
        for i in 0..processed {
            let new_node = Box::new(ListNode::new(sums[i]));
            current.next = Some(new_node);
            current = current.next.as_mut().unwrap();
        }
    }

    result_head.next
}
