use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n < 0 {
            return false;
        }
        for i in 0..n {
            if let Some(power) = (4_i32).checked_pow(i as u32) {
                match (power).cmp(&n) {
                    Ordering::Less => continue,
                    Ordering::Equal => return true,
                    Ordering::Greater => break,
                }
            } else {
                break;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert!(Solution::is_power_of_four(16));
        assert!(Solution::is_power_of_four(1));
    }

    #[test]
    fn ut_false() {
        assert!(Solution::is_power_of_four(1162261466));
        assert!(!Solution::is_power_of_four(0));
        assert!(!Solution::is_power_of_four(15));
        assert!(!Solution::is_power_of_four(-2147483648));
    }
}
