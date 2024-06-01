use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut set_nums = HashSet::new();

        for num in nums.iter() {
            if !set_nums.insert(num) {
                result ^= num;
            }
        }

        result
    }
}
