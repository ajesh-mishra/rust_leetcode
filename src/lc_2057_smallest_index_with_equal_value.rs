pub struct Solution {}

impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .filter(|(index, num)| *index as i32 % 10 == **num)
            .map(|(index, _)| index as i32)
            .take(1)
            .next()
            .unwrap_or(-1)
    }
}
