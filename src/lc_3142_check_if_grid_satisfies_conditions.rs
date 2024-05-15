use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        if !grid[0]
            .iter()
            .zip(grid[0].iter().skip(1))
            .map(|(x, y)| x != y)
            .all(|x| x)
        {
            return false;
        }

        let m = grid.len();
        let n = grid[0].len();

        for j in 0..n {
            if (0..m).map(|i| grid[i][j]).collect::<HashSet<i32>>().len() != 1 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert!(Solution::satisfies_conditions(vec![
            vec![1, 0, 2],
            vec![1, 0, 2]
        ]));
    }
    #[test]
    fn ut_false() {
        assert!(!Solution::satisfies_conditions(vec![
            vec![1, 1, 1],
            vec![0, 0, 0]
        ]));
        assert!(!Solution::satisfies_conditions(vec![
            vec![1],
            vec![2],
            vec![3]
        ]));
    }
}
