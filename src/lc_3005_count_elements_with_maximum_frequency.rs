use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let map_nums = nums.iter().fold(HashMap::new(), |mut acc, num| {
            acc.entry(num).and_modify(|x| *x += 1).or_insert(1);
            acc
        });

        let max_frequency = *map_nums.values().max().unwrap();
        let max_frequency_count = map_nums.values().filter(|&&x| x == max_frequency).count() as i32;

        max_frequency * max_frequency_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
    }
}
