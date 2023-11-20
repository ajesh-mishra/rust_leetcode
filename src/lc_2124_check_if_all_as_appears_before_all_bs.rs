pub struct Solution {}

impl Solution {
    pub fn check_string(s: String) -> bool {
        s.rfind('a') < s.find('b') || s.find('b').is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert!(Solution::check_string("aaabbb".to_owned()));
        assert!(Solution::check_string("bbb".to_owned()));
        assert!(Solution::check_string("a".to_owned()));
    }

    #[test]
    fn ut_false() {
        assert!(!Solution::check_string("abab".to_owned()));
    }
}