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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut list = vec![];
        let mut prev_node = None;
        
        while let Some(node) = head {
            list.push(node.val);
            head = node.next;
        }

        for (index, &val) in list.iter().rev().enumerate() {
            if index + 1 == n as usize {
                continue;
            }
            let mut node = ListNode::new(val);
            if prev_node.is_some() {
                node.next = prev_node;
            }
            prev_node = Some(Box::new(node));
        }
        
        prev_node
    }
}

