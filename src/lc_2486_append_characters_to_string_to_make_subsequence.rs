pub struct Solution {}

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let (mut i, mut j) = (0, 0);

        while i < s.len() && j < t.len() {
            if s.get(i..i + 1).unwrap() == t.get(j..j + 1).unwrap() {
                j += 1;
            }
            i += 1;
        }

        (t.len() - j) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::append_characters("coaching".to_string(), "coding".to_string()),
            4
        );
        assert_eq!(
            Solution::append_characters("abcde".to_string(), "a".to_string()),
            0
        );
        assert_eq!(
            Solution::append_characters("z".to_string(), "abcde".to_string()),
            5
        );
    }
}
