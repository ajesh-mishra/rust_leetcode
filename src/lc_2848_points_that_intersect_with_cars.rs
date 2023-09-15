use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        nums.iter()
            .map(|x| x[0]..=x[1])
            .flatten()
            .collect::<HashSet<i32>>()
            .len() as _
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::number_of_points(vec![vec![3, 6], vec![1, 5], vec![4, 7]]),
            7
        );
        assert_eq!(Solution::number_of_points(vec![vec![1, 3], vec![5, 8]]), 7);
    }
}
