pub struct Solution {}

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len == 1 {
            return vec![0];
        }
        let mut result = vec![];
        let mut left = 0;
        let mut right: i32 = nums.iter().skip(1).sum();

        result.push(right);

        nums.windows(2).for_each(|num| {
            left += num[0];
            right -= num[1];
            result.push((left - right).abs());
        });

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_positive() {
        assert_eq!(
            Solution::left_right_difference(vec![10, 4, 8, 3]),
            vec![15, 1, 11, 22]
        );
        assert_eq!(
            Solution::left_right_difference(vec![1]),
            vec![0]
        );
    }
}
