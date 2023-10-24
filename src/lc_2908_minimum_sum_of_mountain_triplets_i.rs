pub struct Solution {}

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut min_sum = i32::MAX;

        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                for k in j + 1..len {
                    if nums[i] < nums[j] && nums[j] > nums[k] {
                        min_sum = min_sum.min(nums[i] + nums[j] + nums[k]);
                    }
                }
            }
        }
        if min_sum == i32::MAX {
            return -1;
        }
        min_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_positive() {
        assert_eq!(Solution::minimum_sum(vec![8, 6, 1, 5, 3]), 9);
        assert_eq!(Solution::minimum_sum(vec![5, 4, 8, 7, 10, 2]), 13);
    }
    #[test]
    fn ut_negetive() {
        assert_eq!(Solution::minimum_sum(vec![6, 5, 4, 3, 4, 5]), -1);
    }
}
