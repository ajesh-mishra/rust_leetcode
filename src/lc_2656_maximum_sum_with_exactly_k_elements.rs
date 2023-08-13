pub struct Solution {}

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let x = nums.into_iter().max().unwrap();
        (0..k).fold(0, |mut acc, count| {
            acc += x + count;
            acc
        })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_one() {
        assert_eq!(Solution::maximize_sum(vec![1, 2, 3, 4, 5], 3), 18);
        assert_eq!(Solution::maximize_sum(vec![5, 5, 5], 2), 11);
    }
}
