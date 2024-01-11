pub struct Solution {}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let start = left as usize;
        let end = right as usize;
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        words
            .get(start..=end)
            .unwrap()
            .iter()
            .filter(|word| {
                let first = word.chars().next().unwrap();
                let last = word.chars().next_back().unwrap();
                vowels.contains(&first) && vowels.contains(&last)
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::vowel_strings(
                vec!["are".to_owned(), "amy".to_owned(), "u".to_owned()],
                0,
                2
            ),
            2
        );
        assert_eq!(
            Solution::vowel_strings(
                vec![
                    "hey".to_owned(),
                    "aeo".to_owned(),
                    "mu".to_owned(),
                    "ooo".to_owned(),
                    "artro".to_owned()
                ],
                1,
                4
            ),
            3
        );
    }
}
