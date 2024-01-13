pub struct Solution {}

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid[0].len();
        let mut result = vec![0; n];

        for row in grid.iter() {
            for (index, num) in row.iter().enumerate() {
                result[index] = result[index].max(format!("{}", num).len() as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::find_column_width(vec![vec![1], vec![22], vec![333]]),
            vec![3]
        );
        assert_eq!(
            Solution::find_column_width(vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2]]),
            vec![3, 1, 2]
        );
    }
}
