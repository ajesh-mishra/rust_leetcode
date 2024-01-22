use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let (length, sum_nums, sum_nums_set) = (
            nums.len(),
            nums.iter().sum::<i32>(),
            nums.iter()
                .collect::<HashSet<&i32>>()
                .iter()
                .copied()
                .sum::<i32>(),
        );
        vec![
            sum_nums - sum_nums_set,
            (length * (length + 1) / 2) as i32 - sum_nums_set,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }
}
