use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
        amount.sort();
        let amount_sum = amount.iter().sum::<i32>();
        if let [x, y, z] = amount[..] {
            match (x + y).cmp(&z) {
                Ordering::Greater => amount_sum / 2 + amount_sum % 2,
                _ => z,
            }
        } else {
            unreachable!()
        }
    }
}
