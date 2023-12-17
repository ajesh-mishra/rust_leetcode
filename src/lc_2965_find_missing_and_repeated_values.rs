use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let len = grid.len() * grid[0].len();
        let mut set_grid = HashSet::new();
        let mut result = vec![];
        let flat_grid = grid.iter().flatten().copied().collect::<Vec<i32>>();
        let full_grid = (1..=len).map(|x| x as i32).collect::<HashSet<i32>>();

        for &element in flat_grid.iter() {
            if !set_grid.insert(element) {
                result.push(element);
            }
        }

        let diff = full_grid
            .difference(&set_grid)
            .copied()
            .collect::<Vec<i32>>();

        result.extend(diff.iter());

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]),
            vec![2, 4]
        );
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![
                vec![9, 1, 7],
                vec![8, 9, 2],
                vec![3, 4, 6]
            ]),
            vec![9, 5]
        );
    }
}
