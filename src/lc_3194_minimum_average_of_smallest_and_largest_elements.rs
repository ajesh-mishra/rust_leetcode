pub struct Solution {}

impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        let (mut i, mut j, mut average) = (0, nums.len() - 1, f64::MAX);
        nums.sort();

        while i < j {
            average = ((nums[i] + nums[j]) as f64 / 2.0).min(average);
            i += 1;
            j -= 1;
        }

        average
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]),
            5.5
        );
        assert_eq!(Solution::minimum_average(vec![1, 9, 8, 3, 10, 5]), 5.5);
        assert_eq!(Solution::minimum_average(vec![1, 2, 3, 7, 8, 9]), 5.0);
    }
}
