use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut result = nums
            .iter()
            .enumerate()
            .filter(|(_, num)| **num == key)
            .flat_map(|(index, _)| {
                let index = index as i32;
                (index - k..=index + k)
                    .filter(|&x| x >= 0 && x < nums.len() as i32)
                    .collect::<Vec<i32>>()
            })
            .collect::<HashSet<i32>>()
            .iter()
            .copied()
            .collect::<Vec<i32>>();

        result.sort();
        result
    }
}
