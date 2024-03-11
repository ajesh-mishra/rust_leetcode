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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let left = dfs(n.left.clone(), diameter);
                let right = dfs(n.right.clone(), diameter);

                *diameter = (left + right).max(*diameter);
                left.max(right) + 1
            } else {
                0
            }
        }

        let mut diameter = 0;
        dfs(root, &mut diameter);
        
        diameter
    }
}
