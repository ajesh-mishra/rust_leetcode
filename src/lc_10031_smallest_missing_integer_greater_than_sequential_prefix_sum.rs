pub struct Solution {}

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sequential_prefix = 0;

        for i in 1..nums.len() {
            if nums[i - 1] + 1 != nums[i] {
                sequential_prefix = i;
                break;
            }
        }

        let sum_sequential_prefix: i32 = if sequential_prefix == 0 {
            nums.iter().sum()
        } else {
            nums.get(..sequential_prefix).unwrap().iter().sum()
        };

        for i in sum_sequential_prefix..sum_sequential_prefix + nums.len() as i32 {
            if !nums.contains(&i) {
                return i;
            }
        }

        nums.iter().max().unwrap() + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::missing_integer(vec![1, 2, 3, 2, 5]), 6);
        assert_eq!(Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
        assert_eq!(
            Solution::missing_integer(vec![29, 30, 31, 32, 33, 34, 35, 36, 37]),
            297
        );
    }
}
