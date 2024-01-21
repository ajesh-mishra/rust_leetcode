pub struct Solution {}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut subarray = nums.get(1..).unwrap().iter().collect::<Vec<&i32>>();
        subarray.sort();
        nums[0] + subarray.get(0..2).unwrap().iter().copied().sum::<i32>()
    }
}
