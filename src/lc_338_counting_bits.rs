pub struct Solution {}

impl Solution {
    pub fn count_bits_1(n: i32) -> Vec<i32> {
        let mut result = vec![0];
        for i in 1..=n {
            result.push(i.count_ones() as i32);
        }
        result
    }
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..=n).map(|x| x.count_ones() as i32).collect::<Vec<i32>>()
    }
}
