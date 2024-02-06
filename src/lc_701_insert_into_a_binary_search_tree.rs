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
use std::cmp::Ordering::Greater;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }

        fn inner(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
            if let Some(r) = root {
                let mut r = r.borrow_mut();
                match (r.left.is_none(), val.cmp(&r.val), r.right.is_none()) {
                    (_, Greater, true) => r.right = Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                    (_, Greater, false) => inner(&r.right, val),
                    (true, _, _) => r.left = Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                    (false, _, _) => inner(&r.left, val),
                }
            }
        }

        inner(&root, val);
        root
    }
}
