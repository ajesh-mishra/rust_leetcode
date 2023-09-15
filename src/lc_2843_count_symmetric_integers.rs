pub struct Solution {}

impl Solution {
    fn is_symmetric(num: &i32) -> bool {
        let s = format!("{}", num);

        if s.len() % 2 != 0 {
            return false;
        }

        let mid = s.len() / 2;
        let s1 = s.get(0..mid).unwrap();
        let s2 = s.get(mid..).unwrap();

        s1.chars().map(|x| x.to_digit(10).unwrap()).sum::<u32>()
            == s2.chars().map(|x| x.to_digit(10).unwrap()).sum::<u32>()
    }
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        (low..=high).filter(|x| Self::is_symmetric(x)).count() as _
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(Solution::count_symmetric_integers(1, 100), 9);
        assert_eq!(Solution::count_symmetric_integers(1200, 1230), 4);
    }
}
