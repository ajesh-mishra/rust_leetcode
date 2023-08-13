use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() > HashSet::<i32>::from_iter(nums).len()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_positive() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn ut_negetive() {
        assert!(!Solution::contains_duplicate(vec![1,2,3,4]));
    }
}