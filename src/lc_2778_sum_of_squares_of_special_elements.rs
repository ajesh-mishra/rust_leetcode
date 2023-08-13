pub struct Solution {}

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        nums.iter()
            .enumerate()
            .filter(|(index, &_num)| len as f32 % (index + 1) as f32 == 0.0)
            .fold(0, |mut acc, (_, &num)| {
                acc += num * num;
                acc
            })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_one() {
        assert_eq!(Solution::sum_of_squares(vec![2, 7, 1, 19, 18, 3]), 63);
    }
}
