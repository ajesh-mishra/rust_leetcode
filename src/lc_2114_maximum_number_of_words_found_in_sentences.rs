pub struct Solution {}

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .iter()
            .map(|sentence| sentence.split(' ').collect::<Vec<&str>>().len())
            .max()
            .unwrap() as i32
    }
}
