pub struct Solution {}

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut result = String::from("");
        for c in s.chars() {
            if result.ends_with(format!("{c}{c}").as_str()) {
                continue;
            }
            result.push(c);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::make_fancy_string("leeetcode".to_owned()),
            "leetcode".to_owned()
        );
        assert_eq!(
            Solution::make_fancy_string("aaabaaaa".to_owned()),
            "aabaa".to_owned()
        );
        assert_eq!(
            Solution::make_fancy_string("aab".to_owned()),
            "aab".to_owned()
        );
    }
}
