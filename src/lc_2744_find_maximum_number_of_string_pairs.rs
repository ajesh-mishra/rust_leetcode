pub struct Solution {}

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let len = words.len();
        let mut count = 0;
        let words_reversed: Vec<String> = words
            .iter()
            .map(|word| word.chars().rev().collect::<String>())
            .collect();
        for (i, word) in words.iter().take(len - 1).enumerate() {
            for word_reversed in words_reversed.iter().skip(i + 1) {
                if word == word_reversed {
                    dbg!(word);
                    dbg!(word_reversed);
                    count += 1
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
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec!["aa".to_owned(), "ab".to_owned()]),
            0
        );
    }
}
