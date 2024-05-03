pub struct Solution {}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lower_chars = [false; 26];
        let mut upper_chars = [false; 26];

        word.chars().for_each(|c| match c.is_lowercase() {
            true => lower_chars[(c as u8 - 97) as usize] = true,
            false => upper_chars[(c as u8 - 65) as usize] = true,
        });

        lower_chars
            .iter()
            .zip(upper_chars.iter())
            .filter(|(lower_char, upper_char)| **lower_char && **upper_char)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::number_of_special_chars("aaAbcBC".to_string()), 3);
        assert_eq!(Solution::number_of_special_chars("abc".to_string()), 0);
        assert_eq!(Solution::number_of_special_chars("abBCab".to_string()), 1);
    }
}
