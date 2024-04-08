use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let (mut prev, mut trend, mut count, mut result) = (*nums.first().unwrap(), 0, 0, 0);

        for &num in nums.iter().skip(1) {
            match num.cmp(&prev) {
                Ordering::Greater => {
                    if trend == 1 {
                        count += 1;
                    } else {
                        trend = 1;
                        count = 1;
                    }
                }
                Ordering::Less => {
                    if trend == -1 {
                        count += 1;
                    } else {
                        trend = -1;
                        count = 1;
                    }
                }
                Ordering::Equal => {
                    trend = 0;
                    count = 0;
                }
            }
            result = result.max(count);
            prev = num;
        }
        result + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 2, 1]), 3);
    }
}
