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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nums = vec![];

        fn dp(node: Rc<RefCell<TreeNode>>, nums: &mut Vec<String>, val: String) -> () {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                nums.push(format!("{val}{}", node.val));
            }
            if let Some(n) = &node.left {
                dp(n.clone(), nums, format!("{val}{}", node.val))
            }
            if let Some(n) = &node.right {
                dp(n.clone(), nums, format!("{val}{}", node.val))
            }
        }
        if let Some(n) = root {
            dp(n, &mut nums, String::from(""));
        }
        nums.iter()
            .map(|x| i32::from_str_radix(x, 2).unwrap())
            .sum()
    }
}
