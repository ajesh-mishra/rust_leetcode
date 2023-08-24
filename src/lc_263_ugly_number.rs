pub struct Solution {}

impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n < 1 {
            return false;
        }
        for prime in [2, 3, 5].iter() {
            while n % prime == 0 {
                n /= prime;
            }
        }
        n == 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        assert!(Solution::is_ugly(6));
        assert!(Solution::is_ugly(1));
    }

    #[test]
    fn ut_false() {
        assert!(!Solution::is_ugly(14));
        assert!(!Solution::is_ugly(-2147483648));
    }
}
