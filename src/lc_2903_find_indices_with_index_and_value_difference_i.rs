pub struct Solution {}

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let len = nums.len();
        let index_difference = index_difference as usize;

        if len <= index_difference {
            return vec![-1, -1];
        }

        for i in 0..len - index_difference {
            for j in i + index_difference..len {
                if (nums[i] - nums[j]).abs() >= value_difference as i32 {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_found() {
        assert_eq!(Solution::find_indices(vec![5, 1, 4, 1], 2, 4), vec![0, 3]);
        assert_eq!(Solution::find_indices(vec![2, 1], 0, 0), vec![0, 0]);
        assert_eq!(Solution::find_indices(vec![5, 10], 1, 2), vec![0, 1]);
    }
    #[test]
    fn ut_not_found() {
        assert_eq!(Solution::find_indices(vec![1, 2, 3], 2, 4), vec![-1, -1]);
        assert_eq!(Solution::find_indices(vec![0], 100, 50), vec![-1, -1]);
    }
}
