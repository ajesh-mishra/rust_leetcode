use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums1_set = nums1.iter().copied().collect::<HashSet<i32>>();
        let nums2_set = nums2.iter().copied().collect::<HashSet<i32>>();

        if let Some(&minimum_digit) = nums1_set.intersection(&nums2_set).min() {
            return minimum_digit;
        }

        let digit_1 = *nums1.iter().min().unwrap();
        let digit_2 = *nums2.iter().min().unwrap();

        let num_1 = digit_1 * 10 + digit_2;
        let num_2 = digit_2 * 10 + digit_1;

        num_1.min(num_2)
    }
}
