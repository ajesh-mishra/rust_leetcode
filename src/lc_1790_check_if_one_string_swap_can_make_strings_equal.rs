pub struct Solution {}

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut mismatch_count = 0;
        let mut first_mismatch = String::from("");

        for (c1, c2) in s1.chars().zip(s2.chars()) {
            match (c1 == c2, mismatch_count) {
                (true, _) => {}
                (false, 0) => {
                    first_mismatch = format!("{}{}", c1, c2);
                    mismatch_count += 1;
                }
                (false, 1) => {
                    if first_mismatch != format!("{}{}", c2, c1) {
                        return false;
                    }
                    mismatch_count += 1;
                }
                (false, _) => return false,
            }
        }

        matches!(mismatch_count, 0 | 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert!(Solution::are_almost_equal(
            "bank".to_owned(),
            "kanb".to_owned()
        ));
        assert!(Solution::are_almost_equal(
            "kelb".to_owned(),
            "kelb".to_owned()
        ));
    }

    #[test]
    fn ut_false() {
        assert!(!Solution::are_almost_equal(
            "attack".to_owned(),
            "defend".to_owned()
        ));
        assert!(!Solution::are_almost_equal(
            "aa".to_owned(),
            "ac".to_owned()
        ));
    }
}
