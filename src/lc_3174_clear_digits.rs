pub struct Solution {}

impl Solution {
    pub fn clear_digits(s: String) -> String {
        fn inner(s: String) -> String {
            let mut pos: Option<usize> = None;

            for (index, c) in s.char_indices() {
                if c.is_ascii_digit() {
                    pos = Some(index);
                    break;
                }
            }

            if let Some(p) = pos {
                if p <= 1 {
                    inner(s.get(p + 1..).unwrap().to_string())
                } else {
                    inner(format!(
                        "{}{}",
                        s.get(..p - 1).unwrap(),
                        s.get(p + 1..).unwrap()
                    ))
                }
            } else {
                s
            }
        }

        inner(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::clear_digits("abc".to_string()), "abc".to_string());
        assert_eq!(Solution::clear_digits("cb34".to_string()), "".to_string());
        // assert_eq!(Solution::clear_digits("".to_string()), "".to_string());
    }
}
