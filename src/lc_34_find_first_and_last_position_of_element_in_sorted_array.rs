pub struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start = if let Some(pos) = nums.iter().position(|&x| x == target) {
            pos
        } else {
            return vec![-1, -1];
        };
        let end = (start..nums.len()).filter(|&x| nums[x] == target).count();
        vec![start as i32, (start + end - 1) as i32]
    }
}
