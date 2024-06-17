pub struct Solution {}

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut count = 0;

        for i in 0..hours.len() - 1 {
            for j in i + 1..hours.len() {
                if (hours[i] + hours[j]) % 24 == 0 {
                    count += 1;
                }
            }
        }

        count
    }
}
