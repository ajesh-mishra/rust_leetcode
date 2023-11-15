use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    fn get_map<T>(elements: Vec<T>) -> Vec<i32>
    where
        T: Eq,
        T: std::hash::Hash,
    {
        let mut map = HashMap::new();
        for element in elements.iter() {
            map.entry(element).and_modify(|x| *x += 1).or_insert(1);
        }
        map.values().map(|&x| x).collect::<Vec<i32>>()
    }

    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let rank_map = Solution::get_map(ranks);
        let suit_map = Solution::get_map(suits);

        if suit_map.len() == 1 && suit_map[0] == 5 {
            "Flush".to_owned()
        } else if *rank_map.iter().max().unwrap() >= 3 {
            "Three of a Kind".to_owned()
        } else if *rank_map.iter().max().unwrap() >= 2 {
            "Pair".to_owned()
        } else {
            "High Card".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::best_hand(vec![13, 2, 3, 1, 9], vec!['a', 'a', 'a', 'a', 'a']),
            "Flush".to_owned()
        );
        assert_eq!(
            Solution::best_hand(vec![4, 4, 2, 4, 4], vec!['d', 'a', 'a', 'b', 'c']),
            "Three of a Kind".to_owned()
        );
        assert_eq!(
            Solution::best_hand(vec![10, 10, 2, 12, 9], vec!['a', 'b', 'c', 'a', 'd']),
            "Pair".to_owned()
        );
    }
}
