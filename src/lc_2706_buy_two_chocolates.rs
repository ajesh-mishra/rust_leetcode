pub struct Solution {}

impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort_by(|a, b| a.cmp(b));

        let money_spent = prices.iter().take(2).sum::<i32>();
        let left_over = money - money_spent;

        if left_over >= 0 {
            left_over
        } else {
            money
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::buy_choco(vec![1, 2, 2], 3), 0);
        assert_eq!(Solution::buy_choco(vec![3, 2, 3], 3), 3);
    }
}
