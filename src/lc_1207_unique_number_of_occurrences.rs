use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut count_map: HashMap<i32, u32> = HashMap::new();

        for n in arr {
            count_map.entry(n).and_modify(|x| *x += 1).or_insert(1);
        }

        count_map.len() == count_map.values().copied().collect::<HashSet<u32>>().len()
    }
}
