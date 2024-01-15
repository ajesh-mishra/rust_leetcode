pub struct Solution {}

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        _num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        if num_ones > k {
            k
        } else if num_ones + num_zeros > k {
            num_ones
        } else {
            (num_ones * 2) + num_zeros - k
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 0, 2), 2);
        assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 0, 4), 3);
    }
}
