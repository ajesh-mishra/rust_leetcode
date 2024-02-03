use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut ring_map = HashMap::new();

        for ring in rings.chars().zip(rings.chars().skip(1)) {
            if let Some(count) = ring.1.to_digit(10) {
                ring_map
                    .entry(count)
                    .and_modify(|x: &mut HashSet<char>| {
                        x.insert(ring.0);
                    })
                    .or_insert([ring.0].iter().copied().collect::<HashSet<char>>());
            }
        }

        ring_map
            .values()
            .map(|x| x.len())
            .filter(|&x| x == 3)
            .count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::count_points(String::from("B0B6G0R6R0R6G9")), 1);
        assert_eq!(Solution::count_points(String::from("B0R0G0R9R0B0G0")), 1);
        assert_eq!(Solution::count_points(String::from("G4")), 0);
    }
}