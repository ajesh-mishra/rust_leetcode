pub struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let (mut maximum_depth, mut depth) = (0, 0);

        for c in s.chars() {
            match c {
                '(' => {
                    depth += 1;
                    maximum_depth = maximum_depth.max(depth)
                }
                ')' => depth -= 1,
                _ => {}
            }
        }

        maximum_depth
    }
}
