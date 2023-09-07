pub struct Solution {}

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut result = usize::MIN;
        let mut prev: Option<usize> = None;

        for (index, char) in format!("{:b}", n).char_indices() {
            if char == '1' {
                if let Some(x) = prev {
                    result = result.max(index - x);
                }
                prev = Some(index);
            }
        }

        result as _
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(Solution::binary_gap(22), 2);
        assert_eq!(Solution::binary_gap(8), 0);
        assert_eq!(Solution::binary_gap(5), 2);
    }
}
