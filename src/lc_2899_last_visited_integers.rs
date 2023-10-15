pub struct Solution {}

impl Solution {
    pub fn last_visited_integers(words: Vec<String>) -> Vec<i32> {
        let mut last_int: Option<usize> = None;
        let (mut stack, mut result) = (vec![], vec![]);

        for (index, word) in words.iter().enumerate() {
            if word == "prev" {
                let pos = if let Some(p) = last_int {
                    stack.len() - (index - p)
                } else {
                    stack.len()
                };
                result.push(*stack.get(pos).unwrap_or(&-1));
            } else {
                stack.push(word.parse::<i32>().unwrap());
                last_int = Some(index);
            }
        }

        result
    }
}
