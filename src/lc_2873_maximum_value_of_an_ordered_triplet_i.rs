pub struct Solution {}

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let len = nums.len();
        let mut result = 0;
        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                for k in j + 1..len {
                    let curr = (nums[i] as i64 - nums[j] as i64) * nums[k] as i64;
                    result = result.max(curr);
                }
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
        assert_eq!(Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
        assert_eq!(Solution::maximum_triplet_value(vec![1, 2, 3]), 0);
    }
}
