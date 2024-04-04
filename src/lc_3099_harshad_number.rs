pub struct Solution {}

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let sum = format!("{}", x).chars().fold(0, |mut acc, c| {
            acc += c.to_digit(10).unwrap();
            acc
        }) as i32;

        if x % sum == 0 {
            sum
        } else {
            -1
        }
    }
}
