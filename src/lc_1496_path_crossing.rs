use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut points = vec![(0, 0)];

        for p in path.chars() {
            let last = points.last().unwrap();
            let new_point = match p {
                'N' => (last.0, last.1 + 1),
                'S' => (last.0, last.1 - 1),
                'E' => (last.0 - 1, last.1),
                'W' => (last.0 + 1, last.1),
                _ => { unreachable!() }
            };
            points.push(new_point);
        }

        points.len() != points.into_iter().collect::<HashSet<(i32, i32)>>().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert!(!Solution::is_path_crossing(String::from("NES")));
        assert!(Solution::is_path_crossing(String::from("NESWW")));
    }
}