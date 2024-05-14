pub struct Solution {}

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }

        let mut has_vowels = false;
        let mut has_consonants = false;

        for c in word.chars() {
            match c {
                '0'..='9' => {}
                'a' | 'e' | 'i' | 'o' | 'u' => has_vowels = true,
                'A' | 'E' | 'I' | 'O' | 'U' => has_vowels = true,
                'a'..='z' | 'A'..='Z' => has_consonants = true,
                _ => return false,
            }
        }

        has_vowels && has_consonants
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert!(Solution::is_valid("234Adas".to_owned()));
    }
    #[test]
    fn ut_false() {
        assert!(!Solution::is_valid("b3".to_owned()));
        assert!(!Solution::is_valid("a3$e".to_owned()));
    }
}
