pub struct Solution {}

impl Solution {
    fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }

        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }

        for i in (5..(n as f32).sqrt().ceil() as u32).step_by(6) {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
        }

        true
    }

    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let len = nums.len();
        let mut result = 0;
        for i in 0..len {
            for j in 0..len {
                if i != j && j != len - i - 1 {
                    continue;
                }
                if Self::is_prime(nums[i][j] as u32) {
                    result = result.max(nums[i][j]);
                }
            }
        }
        result
    }
}
