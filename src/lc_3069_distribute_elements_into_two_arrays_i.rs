pub struct Solution {}

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut left = vec![nums[0]];
        let mut right = vec![nums[1]];

        for &num in nums.iter().skip(2) {
            if left.last().unwrap() > right.last().unwrap() {
                left.push(num);
            } else {
                right.push(num);
            }
        }

        left.extend(right);
        left
    }
}
