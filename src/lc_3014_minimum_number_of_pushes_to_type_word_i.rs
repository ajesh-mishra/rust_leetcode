pub struct Solution {}

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let (q, r) = (word.len() / 8, word.len() % 8);
        (r * (q + 1)) as i32 + (1..=q).map(|x| x as i32 * 8).sum::<i32>()
    }
}
