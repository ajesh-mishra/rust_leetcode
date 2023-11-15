pub struct Solution {}

impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let (mut strongest_xor, length) = (0, nums.len());
        for i in 0..length {
            for j in 0..length {
                if (nums[i] - nums[j]).abs() <= nums[i].min(nums[j]) {
                    strongest_xor = strongest_xor.max(nums[i] ^ nums[j]);
                }
            }
        }
        strongest_xor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]), 7);
        assert_eq!(Solution::maximum_strong_pair_xor(vec![5, 6, 25, 30]), 7);
        assert_eq!(Solution::maximum_strong_pair_xor(vec![10, 100]), 0);
        assert_eq!(Solution::maximum_strong_pair_xor(vec![1, 2, 2, 1, 2]), 3);
    }
}
