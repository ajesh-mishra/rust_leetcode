pub struct Solution {}

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut reference_value = -1;

        for (index, (&a, &b)) in nums
            .iter()
            .step_by(2)
            .zip(nums.iter().skip(1).step_by(2))
            .enumerate()
        {
            if index == 0 {
                reference_value = a + b;
                continue;
            }
            if a + b != reference_value {
                break;
            }
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::max_operations(vec![3, 2, 1, 4, 5]), 2);
        assert_eq!(Solution::max_operations(vec![3, 2, 6, 1, 4]), 1);
        assert_eq!(
            Solution::max_operations(vec![2, 2, 3, 2, 4, 2, 3, 3, 1, 3]),
            1
        );
    }
}
