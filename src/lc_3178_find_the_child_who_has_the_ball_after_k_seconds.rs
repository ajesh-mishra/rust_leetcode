pub struct Solution {}

impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let n = n - 1;

        if n > k {
            return k;
        }

        let oscillations = k / n;
        let reminder = k % n;

        if oscillations % 2 == 0 {
            return reminder;
        }

        n - reminder
    }
}
