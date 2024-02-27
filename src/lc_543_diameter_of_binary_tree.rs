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
        fn dfs(node: Rc<RefCell<TreeNode>>, diameter: &mut i32) -> i32 {
            let n = node.borrow();
            if n.left.is_none() && n.right.is_none() {
                return 0;
            }
            let (mut left, mut right) = (0, 0);
            
            if let Some(l) = n.left.clone() {
                left = 1 + dfs(l, diameter);
            }
            
            if let Some(r) = n.right.clone() {
                right = 1 + dfs(r, diameter);
            }
            
            *diameter = (left + right).max(*diameter);
            left.max(right)
        }

        let mut diameter = 0;

        if let Some(r) = root {
            dfs(r, &mut diameter);
        }

        diameter
    }
}
