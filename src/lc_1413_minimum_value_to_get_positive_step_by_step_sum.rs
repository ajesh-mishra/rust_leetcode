pub struct Solution {}

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let (mut minimum_value, mut prev) = (0, 0);
        for num in nums.iter() {
            prev += num;
            minimum_value = minimum_value.min(prev);
        }
        (minimum_value - 1).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
        assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
        assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
    }
}
