pub struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (mut s_chars, mut t_chars) = (s.chars(), t.chars());
        let mut result = String::from("");
        while let Some(sc) = s_chars.next() {
            while let Some(tc) = t_chars.next() {
                if sc == tc {
                    result.push(sc);
                    break;
                }
            }
        }
        s == result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert!(Solution::is_subsequence(
            "abc".to_owned(),
            "ahbgdc".to_owned()
        ));
    }
    #[test]
    fn ut_false() {
        assert!(!Solution::is_subsequence(
            "axc".to_owned(),
            "ahbgdc".to_owned()
        ));
    }
}
