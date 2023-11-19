pub struct Solution {}

impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let mut s1_chars = s1.chars();
        let mut s2_chars = s2.chars();
        let mut s3_chars = s3.chars();

        let mut common = 0;

        while let (Some(s1_char), Some(s2_char), Some(s3_char)) =
            (s1_chars.next(), s2_chars.next(), s3_chars.next())
        {
            if s1_char == s2_char && s2_char == s3_char {
                common += 1;
                continue;
            } else if common == 0 {
                return -1;
            } else {
                break;
            }
        }

        if common == 0 {
            return 0;
        }

        (s1.len() + s2.len() + s3.len() - (3 * common)) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert_eq!(
            Solution::find_minimum_operations("abc".to_owned(), "abb".to_owned(), "ab".to_owned()),
            2
        )
    }

    #[test]
    fn ut_false() {
        assert_eq!(
            Solution::find_minimum_operations("dac".to_owned(), "bac".to_owned(), "cac".to_owned()),
            -1
        )
    }
}
