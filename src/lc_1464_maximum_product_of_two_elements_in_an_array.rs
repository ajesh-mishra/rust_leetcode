pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut nums_minus_one = nums.iter().map(|&x| x - 1).collect::<Vec<i32>>();
        nums_minus_one.sort_by(|a, b| b.cmp(a));
        nums_minus_one[0] * nums_minus_one[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
    }
}
