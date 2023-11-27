pub struct Solution {}

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .iter()
            .enumerate()
            .filter(|(_, word)| word.contains(x.to_string().as_str()))
            .map(|(index, _)| index as i32)
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::find_words_containing(vec!["leet".to_string(), "code".to_string()], 'e'),
            vec![0, 1]
        );
    }
}
