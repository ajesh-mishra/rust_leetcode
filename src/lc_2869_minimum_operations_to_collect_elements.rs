use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut collection = HashSet::new();
        for (index, &num) in nums.iter().rev().enumerate() {
            if num > k {
                continue;
            }
            collection.insert(num);
            if collection.len() as i32 == k {
                return index as i32 + 1;
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
        assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 2), 4);
        assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 5), 5);
        assert_eq!(Solution::min_operations(vec![3, 2, 5, 3, 1], 3), 4);
    }
}
