pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_count = vec![0; 26];
        for c in s.chars() {
            let pos = c as u32 - 97;
            s_count[pos as usize] += 1;
        }
        for c in t.chars() {
            let pos = c as u32 - 97;
            if s_count[pos as usize] == 0 {
                return false;
            }
            s_count[pos as usize] -= 1;
        }
        true
    }
}