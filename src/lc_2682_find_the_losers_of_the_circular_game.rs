pub struct Solution {}

impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut scores = vec![0; n as usize + 1];
        let (mut round, mut player) = (1, 1);
        scores[0] = 1; 
        scores[1] = 1;

        while scores[player as usize] <= 1 {
            let pos = player + (round * k) % n;
            player = if pos > n { pos - n } else { pos };
            scores[player as usize] += 1;
            round += 1;
        }

        scores
            .iter()
            .enumerate()
            .filter(|(_, &score)| score == 0)
            .map(|(index, _)| index as i32)
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(Solution::circular_game_losers(4, 4), vec![2, 3, 4]);
        assert_eq!(Solution::circular_game_losers(5, 2), vec![4, 5]);
        assert_eq!(Solution::circular_game_losers(4, 3), vec![]);
    }

    #[test]
    fn ut_failed() {
        assert_eq!(Solution::circular_game_losers(6, 1), vec![3, 5, 6]);
    }
}
