#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn inner(
            head: Option<Box<ListNode>>,
            result: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if let Some(h) = head {
                let mut curr = ListNode::new(h.val);
                curr.next = result;
                inner(h.next, Some(Box::new(curr)))
            } else {
                result
            }
        }

        if head.is_none() {
            None
        } else {
            inner(head, None)
        }
    }
}
