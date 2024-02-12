use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(HashMap::new(), |mut map, &num| {
                map.entry(num).and_modify(|x| *x += 1).or_insert(1);
                map
            })
            .iter()
            .filter(|(_, count)| **count > nums.len() / 2)
            .map(|(&num, _)| num)
            .take(1)
            .next()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
