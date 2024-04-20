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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut path: Option<String>, paths: &mut Vec<String>) {
            if let Some(n) =  node {
                let n = n.borrow();
                
                path = if let Some(p) = path {
                    Some(format!("{}->{}", p, n.val))
                } else {
                    Some(format!("{}", n.val))
                };
                
                if n.left.is_none() && n.right.is_none() {
                    paths.push(path.unwrap());
                    return;
                }
                
                if n.left.is_some() {
                    dfs(n.left.clone(), path.to_owned(), paths)
                }
                if n.right.is_some() {
                    dfs(n.right.clone(), path, paths)
                }
            }
        }
        
        let mut paths = vec![];
        dfs(root, None, &mut paths);
        
        paths
    }
}