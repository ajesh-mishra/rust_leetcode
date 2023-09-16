pub struct Solution {}

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut alternate_array = vec![];
        for num in nums.windows(2) {
            alternate_array.push(num[1] - num[0]);
        }
        let (mut prev, mut count, mut result) = (0, 0, 0);
        for &s in alternate_array.iter() {
            if (s == 1 && (prev == -1 || count == 0)) || (s == -1 && prev == 1) {
                count += 1;
                prev = s;
                continue;
            }
            prev = s;
            result = result.max(count);
            if s == 1 && prev != -1 {
                count = 1;
            } else {
                count = 0;
            }
        }
        if count != 0 {
            result = result.max(count);
        }
        if result == 0 {
            return -1;
        }
        result + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(Solution::alternating_subarray(vec![2, 3, 4, 3, 4]), 4);
        assert_eq!(Solution::alternating_subarray(vec![4, 5, 6]), 2);
        assert_eq!(
            Solution::alternating_subarray(vec![42, 43, 44, 43, 44, 43, 44, 45, 46]),
            6
        );
        assert_eq!(
            Solution::alternating_subarray(vec![6, 12, 2, 3, 8, 9, 10, 10, 2, 1]),
            2
        );
    }
    #[test]
    fn ut_negative() {
        assert_eq!(Solution::alternating_subarray(vec![21, 9, 5]), -1);
        assert_eq!(
            Solution::alternating_subarray(vec![14, 30, 29, 49, 3, 23, 44, 21, 26, 52]),
            -1
        );
    }
}
