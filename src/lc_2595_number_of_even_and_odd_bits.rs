pub struct Solution {}

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let (mut even, mut odd) = (0, 0);

        for (index, digit) in format!("{n:b}").chars().rev().enumerate() {
            match (index % 2, digit) {
                (0, '1') => even += 1,
                (_, '1') => odd += 1,
                (_, _) => {}
            }
        }

        vec![even, odd]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::even_odd_bit(17), vec![2, 0]);
        assert_eq!(Solution::even_odd_bit(2), vec![0, 1]);
    }
}
