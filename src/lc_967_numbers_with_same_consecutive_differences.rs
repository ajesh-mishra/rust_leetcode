pub struct Solution {}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        fn dfs(_n: i32, _k: i32, num: i32, result: &mut Vec<i32>) {
            if _n == 0 {
                result.push(num);
                return;
            }
            let last_digit = num % 10;
            let prev_digit = last_digit - _k;
            let next_digit = last_digit + _k;
            if prev_digit >= 0 {
                dfs(_n - 1, _k, num * 10 + prev_digit, result)
            }
            if next_digit < 10 && _k != 0 {
                dfs(_n - 1, _k, num * 10 + next_digit, result)
            }
        }

        let mut result = vec![];
        (1..10).for_each(|x| dfs(n - 1, k, x, &mut result));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::nums_same_consec_diff(3, 7),
            vec![181, 292, 707, 818, 929]
        );
        assert_eq!(
            Solution::nums_same_consec_diff(2, 1),
            vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98]
        );
    }
}
