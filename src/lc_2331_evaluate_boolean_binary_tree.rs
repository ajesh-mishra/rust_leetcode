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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();

                if n.val == 1 || n.val == 0 {
                    return n.val;
                }

                if n.val == 2 {
                    dfs(n.left.clone()) | dfs(n.right.clone())
                } else {
                    dfs(n.left.clone()) & dfs(n.right.clone())
                }
            } else {
                unreachable!();
            }
        }

        dfs(root) == 1
    }
}
