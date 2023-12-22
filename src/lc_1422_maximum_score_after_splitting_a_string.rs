pub struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        (0..s.len() - 1)
            .map(|i| {
                let left_zero_count = s.get(..=i).unwrap().chars().filter(|&x| x == '0').count();
                let right_one_count = s
                    .get(i + 1..)
                    .unwrap()
                    .chars()
                    .filter(|&x| x == '1')
                    .count();
                (left_zero_count + right_one_count) as i32
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::max_score(String::from("011101")), 5);
        assert_eq!(Solution::max_score(String::from("00111")), 5);
        assert_eq!(Solution::max_score(String::from("1111")), 3);
        assert_eq!(Solution::max_score(String::from("00")), 1);
    }
}
