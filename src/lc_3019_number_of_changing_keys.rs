pub struct Solution {}

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        s.chars()
            .zip(s.chars().skip(1))
            .filter(|(x, y)| x.to_lowercase().to_string() != y.to_lowercase().to_string())
            .count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::count_key_changes("aAbBcC".to_owned()), 2);
        assert_eq!(Solution::count_key_changes("AaAaAaaA".to_owned()), 0);
    }
}
