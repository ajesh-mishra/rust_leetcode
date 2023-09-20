pub struct Solution {}

impl Solution {
    pub fn sum_indices_with_k_set_bits_1(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        for (index, &num) in nums.iter().enumerate() {
            if index.count_ones() as i32 == k {
                result += num
            }
        }
        result
    }
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter()
            .enumerate()
            .filter(|(index, _)| index.count_ones() as i32 == k)
            .map(|(_, &num)| num)
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1),
            13
        );
        assert_eq!(
            Solution::sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2),
            1
        );
    }
}
