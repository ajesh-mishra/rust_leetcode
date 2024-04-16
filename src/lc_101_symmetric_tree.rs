#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn bfs(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> Vec<Option<i32>> {
            let mut value = vec![];
            let mut queue = VecDeque::from([node]);

            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();

                if let Some(n) = node {
                    let n = n.borrow();
                    value.push(Some(n.val));
                    if is_left {
                        queue.push_back(n.left.clone());
                        queue.push_back(n.right.clone());
                    } else {
                        queue.push_back(n.right.clone());
                        queue.push_back(n.left.clone());
                    }
                } else {
                    value.push(None);
                }
            }

            value
        }

        if let Some(r) = root {
            let r = r.borrow();
            bfs(r.left.clone(), true) == bfs(r.right.clone(), false)
        } else {
            false
        }
    }
}
