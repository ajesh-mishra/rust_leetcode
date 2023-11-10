pub struct Solution {}

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let len = num.len();
        let mut pos = 0;

        for (index, c) in num.chars().rev().enumerate() {
            match c {
                '0' => {}
                _ => {
                    pos = index;
                    break;
                }
            }
        }

        num.get(0..len - pos).unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::remove_trailing_zeros("51230100".to_owned()),
            "512301".to_owned()
        );
        assert_eq!(
            Solution::remove_trailing_zeros("123".to_owned()),
            "123".to_owned()
        );
    }
}
