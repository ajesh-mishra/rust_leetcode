pub struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let length = s.len();

        let even: String = (0..length).map(|x| (x % 2 + 48) as u8 as char).collect();
        let odd: String = (1..=length).map(|x| (x % 2 + 48) as u8 as char).collect();

        let even_mismatch = even.chars().zip(s.chars()).filter(|(a, b)| a != b).count();
        let odd_mismatch = odd.chars().zip(s.chars()).filter(|(a, b)| a != b).count();

        even_mismatch.min(odd_mismatch) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::min_operations(String::from("0100")), 1);
        assert_eq!(Solution::min_operations(String::from("01")), 0);
        assert_eq!(Solution::min_operations(String::from("1111")), 2);
    }
}
