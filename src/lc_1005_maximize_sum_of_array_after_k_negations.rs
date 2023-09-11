pub struct Solution {}

impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort_by(|a, b| a.cmp(b));
        for i in 0..nums.len() {
            if nums[i] < 0 && k > 0 {
                nums[i] *= -1;
                k -= 1
            }
        }
        let mut total = nums.iter().map(|&x| x).sum::<i32>() as i32;
        if k % 2 != 0 {
            let minimum_num = nums.iter().min().unwrap();
            total -= minimum_num * 2;
        }
        total
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3),
            6
        );
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2),
            13
        );
    }
}
