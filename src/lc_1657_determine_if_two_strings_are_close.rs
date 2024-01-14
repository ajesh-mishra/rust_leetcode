use std::iter::zip;

pub struct Solution {}

impl Solution {
    fn map(word: String) -> [i32; 26] {
        let mut map_chars = [0; 26];

        for char in word.chars() {
            let pos = (char as u8 - 97) as usize;
            map_chars[pos] += 1;
        }

        map_chars
    }
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut word1_map = Solution::map(word1);
        let mut word2_map = Solution::map(word2);

        for (a, b) in zip(word1_map, word2_map) {
            if (a == 0 && b == 0) || (a != 0 && b != 0) {
                continue;
            }
            return false;
        }

        word1_map.sort();
        word2_map.sort();

        if word1_map != word2_map {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert!(Solution::close_strings(
            "abc".to_string(),
            "bca".to_string()
        ));
        assert!(Solution::close_strings(
            "cabbba".to_string(),
            "abbccc".to_string()
        ));
    }

    #[test]
    fn ut_false() {
        assert!(!Solution::close_strings("a".to_string(), "aa".to_string()));
        assert!(!Solution::close_strings(
            "abbzzca".to_string(),
            "ababzzcza".to_string()
        ));
        assert!(!Solution::close_strings(
            "uau".to_string(),
            "ssx".to_string()
        ));
    }
}
