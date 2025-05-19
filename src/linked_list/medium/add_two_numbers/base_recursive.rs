use crate::structs::list_node::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    add_two_nums_recursive(l1, l2, 0)
}

/// add_two_nums - Recursive Solution using Two Pointers
///     Time Complexity: O(max(m, n)) — where m and n are the lengths of the two linked lists.
///     Space Complexity: O(max(m, n)) (for the result list) + O(max(m, n)) call stack.
fn add_two_nums_recursive(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }

    let sum =
        l1.as_ref().map_or(0, |node| node.val) + l2.as_ref().map_or(0, |node| node.val) + carry;

    let new_carry = sum / 10;
    let node = Some(Box::new(ListNode {
        val: sum % 10,
        next: add_two_nums_recursive(
            l1.and_then(|node| node.next),
            l2.and_then(|node| node.next),
            new_carry,
        ),
    }));

    node
}
