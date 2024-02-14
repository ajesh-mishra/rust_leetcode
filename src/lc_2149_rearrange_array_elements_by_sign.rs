pub struct Solution {}

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut positives = nums.iter().copied().filter(|&x| x >= 0).map(|x| x);
        let mut negatives = nums.iter().copied().filter(|&x| x < 0).map(|x| x);

        let mut result = vec![];

        while let (Some(positive), Some(negative)) = (positives.next(), negatives.next()) {
            result.push(positive);
            result.push(negative);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]),
            vec![3, -2, 1, -5, 2, -4]
        );
        assert_eq!(Solution::rearrange_array(vec![-1, 1]), vec![1, -1]);
    }
}
