use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::with_capacity(l.len());
        for (&start, &end) in l.iter().zip(r.iter()) {
            let start = start as usize;
            let end = end as usize;
            let mut subarray = nums
                .get(start..=end)
                .unwrap()
                .iter()
                .map(|&x| x)
                .collect::<Vec<i32>>();
            subarray.sort();
            let len = subarray
                .windows(2)
                .map(|x| x.first().unwrap() - x.last().unwrap())
                .collect::<HashSet<i32>>()
                .len();
            if len == 1 {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![4, 6, 5, 9, 3, 7],
                vec![0, 0, 2],
                vec![2, 3, 5]
            ),
            vec![true, false, true]
        );
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
                vec![0, 1, 6, 4, 8, 7],
                vec![4, 4, 9, 7, 9, 10]
            ),
            vec![false, true, false, false, true, true]
        );
    }
}
