pub struct Solution {}

impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        let s = format!("{num}");
        if s.len() == 1 {
            return true;
        }
        !s.ends_with("0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert!(Solution::is_same_after_reversals(526));
        assert!(Solution::is_same_after_reversals(0));
    }
    #[test]
    fn ut_false() {
        assert!(!Solution::is_same_after_reversals(1800));
    }
}