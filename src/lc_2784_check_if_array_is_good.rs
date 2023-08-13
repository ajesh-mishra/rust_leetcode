pub struct Solution {}

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let len = nums.len();
        if len <= 1 {
            return false;
        }
        
        let expected_sum = (((len + 1) * len) / 2) - 1;
        let actual_sum = nums.iter().sum::<i32>() as usize;
        
        if expected_sum != actual_sum {
            return false;
        }

        let actual_occurance = nums.iter().filter(|&&x| x == (len - 1) as i32).count();
        let expected_occurance = 2;

        actual_occurance == expected_occurance
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        assert!(Solution::is_good(vec![1, 3, 3, 2]));
        assert!(Solution::is_good(vec![1, 1]));
    }

    #[test]
    fn ut_false() {
        assert!(!Solution::is_good(vec![1, 3, 2]));
        assert!(!Solution::is_good(vec![3, 4, 4, 1, 2, 1]));
        assert!(!Solution::is_good(vec![1, 4, 5, 4, 3, 3]));
    }
}
