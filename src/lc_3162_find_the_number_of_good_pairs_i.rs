pub struct Solution {}

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        for &i in nums1.iter() {
            for &j in nums2.iter() {
                if i % (j * k) == 0 {
                    count += 1;
                }
            }
        }

        count
    }
}
