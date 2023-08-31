pub struct Solution {}

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut start = 0;
        let mut prev = '_';
        let mut result = vec![];

        for (i, c) in s.char_indices() {
            if i == 0 || c == prev {
                prev = c;
                continue;
            }
            if i - 1 - start > 1 {
                result.push(vec![start as i32, (i - 1) as i32])
            }
            start = i;
            prev = c;
        }

        if s.len() - 1 - start > 1 {
            result.push(vec![start as i32, (s.len() - 1) as i32])
        }

        result
    }
}
