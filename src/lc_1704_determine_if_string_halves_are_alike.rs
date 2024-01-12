pub struct Solution {}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        let count = s
            .char_indices()
            .filter(|(_, char)| vowels.contains(char))
            .fold([0, 0], |mut count, (index, _)| {
                if index < s.len() / 2 {
                    count[0] += 1;
                } else {
                    count[1] += 1;
                }
                count
            });

        count[0] == count[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert!(Solution::halves_are_alike("book".to_owned()));
        assert!(!Solution::halves_are_alike("textbook".to_owned()));
    }
}
