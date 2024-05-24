pub struct Solution {}

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut prev = None;

        for &num in nums.iter() {
            let curr = if num & 1 == 1 { 1 } else { 0 };
            if let Some(p) = prev {
                if p ^ curr != 1 {
                    return false;
                }
            }
            prev = Some(curr);
        }

        true
    }
}
