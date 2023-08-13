pub struct Solution {}

impl Solution {
    pub fn split_words_by_separator_1(words: Vec<String>, separator: char) -> Vec<String> {
        let mut result = vec![];
        for word in words {
            for w in word.split(separator) {
                if !w.is_empty() {
                    result.push(w.to_owned());
                }
            }
        }
        result
    }
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words
            .iter()
            .flat_map(|word| word.split(separator))
            .filter(|word| !word.is_empty())
            .map(|word| word.to_owned())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_normal() {
        assert_eq!(
            Solution::split_words_by_separator(
                vec![
                    "one.two.three".to_owned(),
                    "four.five".to_owned(),
                    "six".to_owned()
                ],
                '.'
            ),
            vec![
                "one".to_owned(),
                "two".to_owned(),
                "three".to_owned(),
                "four".to_owned(),
                "five".to_owned(),
                "six".to_owned()
            ]
        );
    }

    #[test]
    fn ut_special() {
        assert_eq!(
            Solution::split_words_by_separator(
                vec!["$easy$".to_owned(), "$problem$".to_owned()],
                '$'
            ),
            vec!["easy".to_owned(), "problem".to_owned()]
        );
    }

    #[test]
    fn ut_empty() {
        let x: Vec<String> = vec![];
        assert_eq!(
            Solution::split_words_by_separator(vec!["|||".to_owned()], '|'),
            x
        );
    }
}
