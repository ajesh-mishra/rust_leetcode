use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut mapped_nums = HashMap::new();

        nums.iter().for_each(|&x| {
            mapped_nums
                .entry(x)
                .and_modify(|value| *value += 1)
                .or_insert(1);
        });

        mapped_nums
            .values()
            .map(|&value| (value / 2, value % 2))
            .fold(vec![0, 0], |mut acc, value| {
                acc[0] += value.0;
                acc[1] += value.1;
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]),
            vec![3, 1]
        );
        assert_eq!(Solution::number_of_pairs(vec![1, 1]), vec![1, 0]);
        assert_eq!(Solution::number_of_pairs(vec![0]), vec![0, 1]);
    }
}
