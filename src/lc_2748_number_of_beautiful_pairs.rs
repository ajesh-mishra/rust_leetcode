pub struct Solution {}

impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut count = 0;
        let gcd = |a: i32, b: i32| {
            let mut result = a.min(b);
            while result > 0 {
                if a % result == 0 && b % result == 0 {
                    break;
                }
                result -= 1
            }
            result
        };
        for i in 0..len - 1 {
            let first_char = format!("{}", nums[i]).chars().next().unwrap();
            let first_digit = first_char.to_digit(10).unwrap() as i32;
            for j in i + 1..len {
                let last_digit = nums[j] % 10;
                if gcd(first_digit, last_digit) == 1 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::count_beautiful_pairs(vec![2, 5, 1, 4]), 5);
        assert_eq!(Solution::count_beautiful_pairs(vec![11, 21, 12]), 2);
    }
}
