pub struct Solution {}

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        let mut result = vec![];
        words.sort_by(|a, b| a.len().cmp(&b.len()));

        let is_predecessor = |first_word: String, second_word: String| {
            if first_word.len() + 1 != second_word.len() {
                return false;
            }
            let mut first_chars = first_word.chars();
            let mut second_chars = second_word.chars();
            let mut count = 0;
            while let (Some(f), Some(s)) = (first_chars.next(), second_chars.next()) {
                if f != s {
                    break;
                }
                count += 1;
            }
            first_word.get(count..) == second_word.get(count + 1..)
        };

        for pair in words.windows(2) {
            let first_word = pair[0].clone();
            let second_word = pair[1].to_owned();
            if is_predecessor(first_word, second_word) {
                result.push(true);
                continue;
            }
            result.push(false);
        }
        dbg!(result);
        0
    }
}
