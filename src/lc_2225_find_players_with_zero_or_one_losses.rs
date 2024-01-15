use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let winners = matches.iter().map(|x| x[0]).collect::<HashSet<i32>>();
        let mut losers = HashMap::new();

        matches.iter().map(|x| x[1]).for_each(|x| {
            losers.entry(x).and_modify(|count| *count += 1).or_insert(1);
        });

        let mut players_lost_1_match = losers
            .iter()
            .filter(|(_, count)| **count == 1)
            .map(|(&player, _)| player)
            .collect::<Vec<i32>>();

        let losers_set = losers.into_keys().collect::<HashSet<i32>>();
        let mut players_never_lost = winners
            .difference(&losers_set)
            .copied()
            .collect::<Vec<i32>>();

        players_lost_1_match.sort();
        players_never_lost.sort();

        vec![players_never_lost, players_lost_1_match]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::find_winners(vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 8],
                vec![4, 9],
                vec![10, 4],
                vec![10, 9]
            ]),
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
        );
        assert_eq!(
            Solution::find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]]),
            vec![vec![1, 2, 5, 6], vec![]]
        );
    }
}
