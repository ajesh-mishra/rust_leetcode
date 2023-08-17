pub struct Solution {}

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut count = 0;
        for detail in details.iter() {
            if detail.get(11..=12).unwrap().parse::<i32>().unwrap() > 60 {
                count += 1;
            }
        }
        count
    }
}