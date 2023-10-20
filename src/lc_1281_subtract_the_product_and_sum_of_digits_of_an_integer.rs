pub struct Solution {}

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let [product, sum] = format! {"{}", n}.chars().fold([1, 0], |mut acc, num| {
            let num = num.to_digit(10).unwrap() as i32;
            acc[0] *= num;
            acc[1] += num;
            acc
        });
        product - sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::subtract_product_and_sum(234), 15);
        assert_eq!(Solution::subtract_product_and_sum(4421), 21);
    }
}
