pub struct Solution {}

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut drunk = num_bottles;

        while num_bottles >= num_exchange {
            let quotient = num_bottles / num_exchange;
            num_bottles %= num_exchange;
            num_bottles += quotient;
            drunk += quotient
        }

        drunk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}
