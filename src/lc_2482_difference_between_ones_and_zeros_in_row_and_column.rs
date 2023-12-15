pub struct Solution {}

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut result = vec![vec![0; n]; m];

        for x in 0..m {
            for y in 0..n {
                let ones_col = (0..m).map(|i| grid[i][y]).filter(|&x| x == 1).count() as i32;
                let zeros_col = m as i32 - ones_col;
                let ones_row = (0..n).map(|j| grid[x][j]).filter(|&x| x == 1).count() as i32;
                let zeros_row = n as i32- ones_row;
                result[x][y] = ones_row + ones_col - zeros_row - zeros_col;
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
            Solution::ones_minus_zeros(vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]]),
            vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]]
        );
        assert_eq!(
            Solution::ones_minus_zeros(vec![vec![1, 1, 1], vec![1, 1, 1]]),
            vec![vec![5, 5, 5], vec![5, 5, 5]]
        );
    }
}
