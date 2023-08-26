pub struct Solution {}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut count = 0;
        let len = nums.len();
        for i in 0..len - 1 {
            for j in i + 1..len {
                if nums[i] + nums[j] < target {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(Solution::count_pairs(vec![-1, 1, 2, 3, 1], 2), 3);
        assert_eq!(Solution::count_pairs(vec![-6,2,5,-2,-7,-1,3], -2), 10);
    }
}
