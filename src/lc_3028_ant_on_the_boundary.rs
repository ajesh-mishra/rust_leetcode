pub struct Solution {}

impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(mut count, mut running_total), x| {
                running_total += x;
                if running_total == 0 {
                    count += 1;
                }
                (count, running_total)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::return_to_boundary_count(vec![2, 3, -5]), 1);
        assert_eq!(Solution::return_to_boundary_count(vec![3, 2, -3, -4]), 0);
    }
}
