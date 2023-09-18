pub struct Solution {}

impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let mut prev: Option<i32> = None;
        let mut first: Option<i32> = None;
        let mut shift_point: Option<i32> = None;

        for (index, &num) in nums.iter().enumerate() {
            match (prev, first, shift_point) {
                (None, _, _) => {
                    prev = Some(num);
                    first = Some(num);
                }
                (Some(x), _, None) if num > x => prev = Some(num),
                (_, Some(y), None) if num < y => {
                    prev = Some(num);
                    shift_point = Some(index as i32);
                }
                (Some(x), Some(y), Some(_)) if x < num && num < y => prev = Some(num),
                (_, _, _) => return -1,
            }
        }

        if let Some(x) = shift_point {
            nums.len() as i32 - x
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_positive() {
        assert_eq!(Solution::minimum_right_shifts(vec![3, 4, 5, 1, 2]), 2);
    }
    #[test]
    fn ut_zero() {
        assert_eq!(Solution::minimum_right_shifts(vec![1, 3, 5]), 0);
    }
    #[test]
    fn ut_negative() {
        assert_eq!(Solution::minimum_right_shifts(vec![2, 1, 4]), -1);
    }
}
