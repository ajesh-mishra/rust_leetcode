pub struct Solution {}

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut gifts = gifts.iter().map(|&x| x as i64).collect::<Vec<i64>>();

        for _ in 0..k as usize {
            let biggest = gifts.iter().max().unwrap();
            let pos = gifts.iter().position(|x| x == biggest).unwrap();
            gifts[pos] = (gifts[pos] as f64).sqrt() as i64;
        }

        gifts.iter().sum()
    }
}
