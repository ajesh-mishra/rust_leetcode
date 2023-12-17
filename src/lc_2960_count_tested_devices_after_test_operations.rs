pub struct Solution {}

impl Solution {
    pub fn count_tested_devices(mut battery_percentages: Vec<i32>) -> i32 {
        let mut tested_devices = 0;
        let n = battery_percentages.len();
        for index in 0..n {
            if battery_percentages[index] <= 0 {
                continue;
            }
            tested_devices += 1;
            for i in index + 1..n {
                battery_percentages[i] = 0_i32.max(battery_percentages[i] - 1)
            }
        }
        tested_devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::count_tested_devices(vec![1, 1, 2, 1, 3]), 3);
        assert_eq!(Solution::count_tested_devices(vec![0, 1, 2]), 2);
        // assert_eq!(Solution::count_tested_devices(vec!), );
    }
}
