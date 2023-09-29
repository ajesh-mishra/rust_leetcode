pub struct Solution {}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let map = s.chars().fold([false; 26], |mut acc, c| {
            let char_pos = ((c as u8) - 97) as usize;
            acc[char_pos] = true;
            acc
        });
        map.iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .map(|(index, _)| std::char::from_u32(index as u32 + 97).unwrap())
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_owned()),
            "abc".to_owned()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_owned()),
            "acdb".to_owned()
        );
    }
}
