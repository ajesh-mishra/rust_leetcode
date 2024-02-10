use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut set = HashSet::new();
        edges
            .iter()
            .flatten()
            .filter(|&&x| !set.insert(x))
            .take(1)
            .next()
            .unwrap()
            .to_owned()
    }
}
