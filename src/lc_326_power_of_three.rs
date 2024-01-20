use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        fn inner(num: i32, rem: i32) -> bool {
            if rem != 0 {
                return false;
            }
            match num.cmp(&1) {
                Ordering::Less => false,
                Ordering::Equal => true,
                Ordering::Greater => inner(num / 3, num % 3),
            }
        }

        inner(n, 0)
    }
}
