pub struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut zero_vec = vec![0; 26];

        for c in magazine.chars() {
            let pos = c as u8 - 97;
            if pos < 26 {
                zero_vec[pos as usize] += 1;
            }
        }

        for c in ransom_note.chars() {
            let pos = c as u8 - 97;
            if pos < 26 {
                if zero_vec[pos as usize] == 0 {
                    return false;
                }
                zero_vec[pos as usize] -= 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert!(Solution::can_construct("a".to_owned(), "aab".to_owned()));
    }

    #[test]
    fn ut_false() {
        assert!(!Solution::can_construct("a".to_owned(), "b".to_owned()));
        assert!(!Solution::can_construct("aa".to_owned(), "ab".to_owned()));
    }
}
