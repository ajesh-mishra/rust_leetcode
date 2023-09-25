pub struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut map = [0; 26];
        for c in s.chars() {
            let char_pos = (c as u8 - 97) as usize;
            map[char_pos] += 1;
        }
        for c in t.chars() {
            let char_pos = (c as u8 - 97) as usize;
            map[char_pos] -= 1;
            if map[char_pos] == -1 {
                return c;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_owned(), "abcde".to_owned()),
            'e'
        );
    }
}
