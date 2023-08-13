use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for &num in nums.iter() {
            let max_digit = format!("{}", num).chars().fold(0, |max_digit, n| {
                let digit = n.to_digit(10).unwrap();
                max_digit.max(digit)
            });
            map.entry(max_digit)
                .and_modify(|x: &mut Vec<i32>| x.push(num))
                .or_insert(vec![num]);
        }
        let mut max = -1;
        for mut row in map.into_values() {
            if row.len() < 2 {
                continue;
            }
            row.sort_by(|a, b| b.cmp(a));
            max = max.max(row[0] + row[1]);
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_negetive() {
        assert_eq!(Solution::max_sum(vec![1, 2, 3, 4]), -1);
    }

    #[test]
    fn ut_positive() {
        assert_eq!(Solution::max_sum(vec![31, 25, 72, 79, 74]), 146);
    }
}
