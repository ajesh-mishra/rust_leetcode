use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let map = s.chars().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|x| *x += 1).or_insert(1);
            acc
        });

        let mut map = map
            .iter()
            .map(|(&char, &count)| (char, count))
            .collect::<Vec<(char, i32)>>();

        map.sort_by_key(|(_, count)| *count);

        map.iter()
            .rev()
            .flat_map(|(char, count)| {
                format!("{}", *char)
                    .repeat(*count as usize)
                    .chars()
                    .collect::<Vec<_>>()
            })
            .collect::<String>()
    }
}
