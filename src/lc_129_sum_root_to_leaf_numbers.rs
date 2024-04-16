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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut value = 0;

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut path: i32, value: &mut i32) {
            if let Some(n) = node {
                let n = n.borrow();

                path = (path * 10) + n.val;

                if n.left.is_none() && n.right.is_none() {
                    *value += path;
                    return;
                }

                if n.left.is_some() {
                    dfs(n.left.clone(), path, value);
                }
                if n.right.is_some() {
                    dfs(n.right.clone(), path, value);
                }
            }
        }

        dfs(root, 0, &mut value);
        value
    }
}
