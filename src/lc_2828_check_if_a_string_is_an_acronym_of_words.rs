pub struct Solution {}

impl Solution {
    pub fn is_acronym_1(words: Vec<String>, s: String) -> bool {
        let mut result = String::from("");
        for word in words.iter() {
            let first_char = word.chars().next().unwrap();
            result.push(first_char);
        }
        result == s
    }
    pub fn is_acronym_2(words: Vec<String>, s: String) -> bool {
        words.iter().fold(String::from(""), |mut acc, word| {
            let first_char = word.chars().next().unwrap();
            acc.push(first_char);
            acc
        }) == s
    }
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        String::from_utf8(
            words
                .iter()
                .map(|word| word.bytes().next().unwrap())
                .collect::<Vec<u8>>(),
        )
        .unwrap()
            == s
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        assert!(Solution::is_acronym(
            vec!["alice".to_owned(), "bob".to_owned(), "charlie".to_owned()],
            "abc".to_owned()
        ));
    }

    #[test]
    fn ut_false() {
        assert!(!Solution::is_acronym(
            vec!["an".to_owned(), "apple".to_owned()],
            "a".to_owned()
        ));
    }
}
