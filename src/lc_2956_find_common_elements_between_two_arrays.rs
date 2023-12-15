use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let common = nums1
            .iter()
            .copied()
            .collect::<HashSet<i32>>()
            .intersection(&nums2.iter().copied().collect::<HashSet<i32>>()).copied()
            .collect::<Vec<i32>>();

        let nums1_count = nums1.iter().filter(|&x| common.contains(x)).count() as i32;
        let nums2_count = nums2.iter().filter(|&x| common.contains(x)).count() as i32;

        vec![nums1_count, nums2_count]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::find_intersection_values(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6]),
            vec![3, 4]
        );
    }
}
