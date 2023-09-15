pub struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        fn inner(amount: i32, coins: Vec<i32>, count: i32) -> i32 {
            for &coin in coins.iter() {
                match amount - coin {
                    0 => return 1,
                    ..=0 => return 0,
                    x => return count + inner(x, coins, count),
                }
            }
            count
        }
        // let count = 0;
        inner(amount, coins, 0)
        // count
    }
}
