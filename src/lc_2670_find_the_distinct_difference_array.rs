use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn distinct_difference_array_1(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for index in 0..nums.len() {
            let (mut prefix, mut suffix) = (HashSet::new(), HashSet::new());
            for (i, num) in nums.iter().enumerate() {
                if i <= index {
                    prefix.insert(num);
                    continue;
                }
                suffix.insert(num);
            }
            let diff = prefix.len() as i32 - suffix.len() as i32;
            result.push(diff);
        }
        result
    }
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len())
            .map(|index| {
                let prefix = nums
                    .iter()
                    .enumerate()
                    .filter(|(i, _v)| *i <= index)
                    .map(|(_, &v)| v)
                    .collect::<HashSet<i32>>()
                    .len() as i32;
                let suffix = nums
                    .iter()
                    .skip(index + 1)
                    .map(|&v| v)
                    .collect::<HashSet<i32>>()
                    .len() as i32;
                prefix - suffix
            })
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::distinct_difference_array(vec![3, 2, 3, 4, 2]),
            vec![-2, -1, 0, 2, 3]
        );
    }
}
