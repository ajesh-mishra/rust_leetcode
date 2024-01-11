pub struct Solution {}

impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut num_list = format!("{num}").chars().collect::<Vec<char>>();
        num_list.sort();

        let mut even_digits = 0;
        let mut odd_digits = 0;

        for (index, digit) in num_list.iter().enumerate() {
            match index % 2 {
                0 => even_digits = (even_digits * 10) + digit.to_digit(10).unwrap(),
                _ => odd_digits = (odd_digits * 10) + digit.to_digit(10).unwrap(),
            }
        }

        (even_digits + odd_digits) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::split_num(4325), 59);
        assert_eq!(Solution::split_num(687), 75);
    }
}
