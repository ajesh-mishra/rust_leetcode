pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut char_map = vec![0; 52];
        let mut result = 0;
        let mut has_odd = false;

        for c in s.chars() {
            let pos = c as u8;
            let pos = if pos >= 97 {
                pos as usize - 97 + 26
            } else {
                pos as usize - 65
            };
            char_map[pos] += 1;
        }

        for &count in char_map.iter() {
            if count == 1 {
                has_odd = true;
            } else if count & 1 == 1 {
                has_odd = true;
                result += count - 1;
            } else {
                result += count
            }
        }

        if has_odd {
            result += 1;
        }

        result
    }
}
