pub struct Solution {}

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .iter()
            .filter(|&pattern| word.contains(pattern))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::num_of_strings(
                vec![
                    "a".to_owned(),
                    "abc".to_owned(),
                    "bc".to_owned(),
                    "d".to_owned()
                ],
                "abc".to_owned()
            ),
            3
        );
        assert_eq!(
            Solution::num_of_strings(
                vec!["a".to_owned(), "a".to_owned(), "a".to_owned(),],
                "ab".to_owned()
            ),
            3
        );
        assert_eq!(
            Solution::num_of_strings(
                vec!["a".to_owned(), "b".to_owned(), "c".to_owned(),],
                "aaaaabbbbb".to_owned()
            ),
            2
        );
    }
}
