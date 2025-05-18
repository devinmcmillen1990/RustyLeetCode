#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

#[repr(align(64))]
pub struct AlignedListNode {
    pub val: i32,
    pub next: Option<Box<AlignedListNode>>,
}

impl AlignedListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        AlignedListNode { val, next: None }
    }
}

impl Clone for AlignedListNode {
    fn clone(&self) -> Self {
        AlignedListNode {
            val: self.val,
            next: self.next.as_ref().map(|node| node.clone()),
        }
    }
}
