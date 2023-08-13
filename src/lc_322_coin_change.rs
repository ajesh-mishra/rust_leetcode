pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let index = amount as usize;
        let mut result = vec![amount + 1; index + 1];
        
        result[0] = 0;

        for a in 1..=amount as usize {
            for &coin in coins.iter() {
                if a >= coin as usize{
                    let bal = a - coin as usize;
                    result[a] = result[a].min(1 + result[bal]);
                }
            }
        }

        if result[index] == amount + 1 {
            return -1;
        }

        result[index]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_value() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn ut_negetive() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }
}
