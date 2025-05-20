use crate::structs::list_node::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;
    let mut l1 = list1;
    let mut l2 = list2;

    while l1.is_some() && l2.is_some() {
        let list1_value = l1.as_ref().unwrap().val;
        let list2_value = l2.as_ref().unwrap().val;

        if list1_value <= list2_value {
            let mut node = l1.take().unwrap();
            l1 = node.next.take();
            tail.next = Some(node);
        } else {
            let mut node = l2.take().unwrap();
            l2 = node.next.take();
            tail.next = Some(node);
        }

        tail = tail.next.as_mut().unwrap();
    }

    tail.next = if l1.is_some() { l1 } else { l2 };
    head.next
}
