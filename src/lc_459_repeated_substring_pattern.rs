pub struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let len = s.len();

        for i in 1..=(len / 2) {
            if len % i == 0 {
                let substr = &s[0..i];
                let mut calculated = String::from(substr);

                for _ in 1..(len / i) {
                    calculated.push_str(substr);
                }

                if calculated == s {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        assert!(Solution::repeated_substring_pattern("abab".to_string()));
        assert!(Solution::repeated_substring_pattern(
            "abcabcabcabc".to_string()
        ));
    }

    #[test]
    fn ut_false() {
        assert!(!Solution::repeated_substring_pattern("aba".to_string()));
    }
}
