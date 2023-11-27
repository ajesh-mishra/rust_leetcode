pub struct Solution {}

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let mut c = coordinates.chars();
        let x = c.next().unwrap() as u8 - 96;
        let y = c.next().unwrap().to_digit(10).unwrap() as u8;

        (x % 2 == 0) ^ (y % 2 == 0)

        // match (x % 2 == 0, y % 2 == 0) {
        //     (true, true) => false,
        //     (true, false) => true,
        //     (false, true) => true,
        //     (_, _) => false,
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert!(Solution::square_is_white("h3".to_owned()));
    }
    #[test]
    fn ut_false() {
        assert!(!Solution::square_is_white("a1".to_owned()));
        assert!(!Solution::square_is_white("c7".to_owned()));
    }
}