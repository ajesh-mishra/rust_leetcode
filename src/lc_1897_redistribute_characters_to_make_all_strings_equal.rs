pub struct Solution {}

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut char_map = [0; 26];
        words
            .iter()
            .flat_map(|word| word.chars())
            .for_each(|c| {
                let pos = c as u8 - 97;
                char_map[pos as usize] += 1;
            });
        char_map.iter().all(|&x| x % words.len() == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert!(Solution::make_equal(vec![
            "abc".to_owned(),
            "aabc".to_owned(),
            "bc".to_owned()
        ]));
        assert!(!Solution::make_equal(vec!["ab".to_owned(), "a".to_owned()]));
    }
}
