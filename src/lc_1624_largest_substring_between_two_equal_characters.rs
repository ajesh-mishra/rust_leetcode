pub struct Solution {}

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut char_map = vec![vec![]; 26];

        for (index, c) in s.char_indices() {
            let pos = (c as u8 - 97) as usize;
            char_map[pos].push(index);
        }

        let result = char_map.iter().fold(-1, |mut acc, x| {
            if x.len() > 1 {
                acc = acc.max((x.last().unwrap() - x[0] - 1) as i32);
            }
            acc
        });

        if result == -1 {
            return -1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::max_length_between_equal_characters(String::from("aa")),
            0
        );
        assert_eq!(
            Solution::max_length_between_equal_characters(String::from("abca")),
            2
        );
        assert_eq!(
            Solution::max_length_between_equal_characters(String::from("cbzxy")),
            -1
        );
    }
}
