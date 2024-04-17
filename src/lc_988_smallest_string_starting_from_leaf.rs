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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut paths = vec![];

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut path: String, paths: &mut Vec<String>) {
            if let Some(n) = node {
                let n = n.borrow();

                let val = char::from(n.val as u8 + 97);
                path = format!("{}{}", val, path);

                if n.left.is_none() && n.right.is_none() {
                    paths.push(path.to_owned());
                }

                if n.left.is_some() {
                    dfs(n.left.clone(), path.to_owned(), paths);
                }
                if n.right.is_some() {
                    dfs(n.right.clone(), path, paths);
                }
            }
        }

        dfs(root, String::from(""), &mut paths);

        paths.sort();
        paths.first().unwrap().to_owned()
    }
}
