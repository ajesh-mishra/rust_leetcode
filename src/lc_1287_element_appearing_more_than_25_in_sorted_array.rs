pub struct Solution {}

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut prev = None;
        let threshold = (arr.len() / 4) as i32;
        for &a in arr.iter() {
            if let Some(p) = prev {
                if p == a {
                    count += 1;
                } else {
                    count = 1;
                }
            }
            if count > threshold {
                return a;
            }
            prev = Some(a);
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
            6
        );
        assert_eq!(Solution::find_special_integer(vec![1, 1]), 1);
        assert_eq!(Solution::find_special_integer(vec![1]), 1);
    }
}
