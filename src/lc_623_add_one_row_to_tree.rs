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
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let mut x = TreeNode::new(val);
            x.left = root;
            return Some(Rc::new(RefCell::new(x)));
        }

        let result = root.clone();
        let mut queue = VecDeque::from([(root, 1)]);

        while !queue.is_empty() {
            let (node, curr_depth) = queue.pop_front().unwrap();
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                let (original_left, original_right) = (n.left.clone(), n.right.clone());

                if curr_depth + 1 == depth {
                    let mut new_left = TreeNode::new(val);
                    new_left.left = original_left.clone();
                    n.left = Some(Rc::new(RefCell::new(new_left)));

                    let mut new_right = TreeNode::new(val);
                    new_right.right = original_right.clone();
                    n.right = Some(Rc::new(RefCell::new(new_right)));
                }

                if original_left.is_some() {
                    queue.push_back((original_left, curr_depth + 1));
                }
                if original_right.is_some() {
                    queue.push_back((original_right, curr_depth + 1));
                }
            }
        }

        result
    }
}
