use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut nums_set = HashSet::new();
        for &num in nums.iter() {
            if !nums_set.insert(num) {
                return num;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
    #[test]
    fn ut_zero() {
        assert_eq!(Solution::find_duplicate(vec![2, 2, 2, 2, 2]), 2);
    }
}
