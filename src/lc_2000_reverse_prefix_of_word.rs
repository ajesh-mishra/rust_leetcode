pub struct Solution {}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let pos = word.find(ch).unwrap_or(0);
        let left = word.get(..=pos).unwrap().chars().rev().collect::<String>();
        let right = word.get(pos + 1..).unwrap_or("");
        format!("{}{}", left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::reverse_prefix("abcdefd".to_owned(), 'd'),
            "dcbaefd".to_owned()
        );
        assert_eq!(
            Solution::reverse_prefix("xyxzxe".to_owned(), 'z'),
            "zxyxxe".to_owned()
        );
        assert_eq!(
            Solution::reverse_prefix("abcd".to_owned(), 'z'),
            "abcd".to_owned()
        );
    }
}
