use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let (len, mut result) = (nums.len(), 0);
        for window in 1..=len {
            for sub in nums.windows(window) {
                result += sub.iter().collect::<HashSet<&i32>>().len().pow(2);
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
        assert_eq!(Solution::sum_counts(vec![1, 1]), 3);
    }
}
