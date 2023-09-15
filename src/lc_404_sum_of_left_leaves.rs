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
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn inner(root: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            let tree = if let Some(node) = root {
                node.borrow()
            } else {
                return 0;
            };
            let _sum = if tree.left.is_none() && tree.right.is_none() && is_left {
                tree.val
            } else {
                0
            };
            _sum + inner(&tree.left, true) + inner(&tree.right, false)
        }

        inner(&root, false)
    }
}
