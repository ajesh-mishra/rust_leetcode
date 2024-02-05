pub struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        (0..s.len())
            .filter(|&i| {
                let c = s.get(i..i + 1).unwrap();
                let left = s.get(..i).unwrap();
                let right = s.get(i + 1..).unwrap();
                !format!("{left}{right}").contains(c)
            })
            .map(|x| x as i32)
            .take(1)
            .next()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::first_uniq_char(String::from("leetcode")), 0);
        assert_eq!(Solution::first_uniq_char(String::from("loveleetcode")), 2);
        assert_eq!(Solution::first_uniq_char(String::from("aabb")), -1);
    }
}
