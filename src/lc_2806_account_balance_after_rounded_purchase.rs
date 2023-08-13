use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn account_balance_after_purchase_1(purchase_amount: i32) -> i32 {
        let (mut higher_amount, mut lower_amount) = (purchase_amount, purchase_amount);
        let amount_payable: i32 = loop {
            if higher_amount % 10 == 0 {
                break higher_amount;
            }
            if lower_amount % 10 == 0 {
                break lower_amount;
            }
            higher_amount += 1;
            lower_amount -= 1;
        };
        100 - amount_payable
    }
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        let discount = purchase_amount % 10;
        let aprise = 10 - discount;
        let rounded_amount = match discount.cmp(&aprise) {
            Ordering::Greater => purchase_amount + aprise,
            Ordering::Less => purchase_amount - discount,
            Ordering::Equal => purchase_amount + aprise,
        };
        100 - rounded_amount
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(Solution::account_balance_after_purchase(15), 80);
        assert_eq!(Solution::account_balance_after_purchase(11), 90);
    }
}