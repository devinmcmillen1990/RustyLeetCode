use crate::structs::list_node::ListNode;

/*
Add Two Numbers - Medium

You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each
of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.


Example 1:
    2 -> 4 -> 3
    5 -> 6 -> 4
    ------------
    7 -> 0 -> 8

Input: l1 = [2,4,3], l2 = [5,6,4]
Output: [7,0,8]
Explanation: 342 + 465 = 807.

Example 2:
Input: l1 = [0], l2 = [0]
Output: [0]

Example 3:
Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
Output: [8,9,9,9,0,0,0,1]

Constraints:
    The number of nodes in each linked list is in the range [1, 100].
    0 <= Node.val <= 9
    It is guaranteed that the list represents a number that does not have leading zeros.
*/

/// add_two_nums - Iterative Solution using Two Pointers
///     Time Complexity: O(max(m, n)) — where m and n are the lengths of the two linked lists.
///     Space Complexity: O(max(m, n)) — for the resulting linked list.
pub fn add_two_nums_iteratively(
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

pub fn add_two_nums_recursively(
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
