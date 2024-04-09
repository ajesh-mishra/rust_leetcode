pub struct Solution {}

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let target = tickets[k as usize];

        tickets
            .iter()
            .enumerate()
            .map(|(index, &x)| {
                if index <= k as usize {
                    x.min(target)
                } else {
                    x.min(target - 1)
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
}
