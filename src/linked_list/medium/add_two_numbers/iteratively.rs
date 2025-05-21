use crate::structs::list_node::ListNode;

/// add_two_nums - Iterative Solution using Two Pointers
///     Time Complexity: O(max(m, n)) — where m and n are the lengths of the two linked lists.
///     Space Complexity: O(max(m, n)) — for the resulting linked list.
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result_head = Box::new(ListNode::new(0));
    let mut l1_ = l1;
    let mut l2_ = l2;
    let mut carry = 0;
    let mut current = &mut result_head;

    while l1_.is_some() || l2_.is_some() || carry != 0 {
        let x = l1_.as_ref().map_or(0, |node| node.val);
        let y = l2_.as_ref().map_or(0, |node| node.val);
        let sum = x + y + carry;

        carry = sum / 10;

        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();

        l1_ = l1_.and_then(|node| node.next);
        l2_ = l2_.and_then(|node| node.next);
    }

    result_head.next
}
