pub struct Solution {}

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;

        for (index, word1) in words.iter().take(words.len() - 1).enumerate() {
            for word2 in words.iter().skip(index + 1) {
                if word2.starts_with(word1) && word2.ends_with(word1) {
                    count += 1;
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
        assert_eq!(Solution::count_prefix_suffix_pairs(vec!["a".to_owned(),"aba".to_owned(),"ababa".to_owned(),"aa".to_owned()]), 4);
    }
}