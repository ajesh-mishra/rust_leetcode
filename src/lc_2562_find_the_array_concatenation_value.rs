pub struct Solution {}

impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let (mut value, mut start, mut end) = (0, 0, nums.len() - 1);
        while start < end {
            value += format!("{}{}", nums[start], nums[end])
                .parse::<i64>()
                .unwrap();
            start += 1;
            end -= 1;
        }
        if start == end {
            value += nums[start] as i64;
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::find_the_array_conc_val(vec![7, 52, 2, 4]), 596);
        assert_eq!(Solution::find_the_array_conc_val(vec![1]), 1);
        assert_eq!(
            Solution::find_the_array_conc_val(vec![5, 14, 13, 8, 12]),
            673
        );
    }
}
