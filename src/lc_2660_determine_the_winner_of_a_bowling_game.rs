use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    fn calculate_total_score(scores: &Vec<i32>) -> i32 {
        let mut last_ten: Option<usize> = None;
        let mut total_score = 0;
        for (pos, &score) in scores.iter().enumerate() {
            if let Some(x) = last_ten {
                if pos - x <= 2 {
                    total_score += score;
                }
            }
            total_score += score;
            if score == 10 {
                last_ten = Some(pos);
            }
        }
        total_score
    }
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        match Solution::calculate_total_score(&player1)
            .cmp(&Solution::calculate_total_score(&player2))
        {
            Ordering::Equal => 0,
            Ordering::Greater => 1,
            Ordering::Less => 2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_zero() {
        assert_eq!(Solution::is_winner(vec![2, 3], vec![4, 1]), 0)
    }

    #[test]
    fn ut_one() {
        assert_eq!(Solution::is_winner(vec![4, 10, 7, 9], vec![6, 5, 2, 3]), 1)
    }
    
    #[test]
    fn ut_two() {
        assert_eq!(Solution::is_winner(vec![3, 5, 7, 6], vec![8, 10, 10, 2]), 2)
    }
}
