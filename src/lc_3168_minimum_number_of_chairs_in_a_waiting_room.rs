pub struct Solution {}

impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let (mut count, mut max_count) = (0, 0);

        for c in s.chars() {
            match c {
                'E' => {
                    count += 1;
                    max_count = max_count.max(count);
                }
                'L' => count -= 1,
                _ => unreachable!(),
            }
        }

        max_count
    }
}
