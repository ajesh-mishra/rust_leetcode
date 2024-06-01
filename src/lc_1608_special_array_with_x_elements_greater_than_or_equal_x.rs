pub struct Solution {}

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        let (mut prev, length) = (0, nums.len());
        nums.sort();

        for (index, &num) in nums.iter().enumerate() {
            let remaining = (length - index) as i32;
            if remaining <= num && prev < remaining {
                return remaining;
            }
            prev = num
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::special_array(vec![3, 5]), 2);
        assert_eq!(Solution::special_array(vec![0, 0]), -1);
        assert_eq!(Solution::special_array(vec![0, 4, 3, 0, 4]), 3);
    }
}
