pub struct Solution {}

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let l = matrix.len();

        if l == 1 || matrix[0].len() == 1 {
            return true;
        }

        for (i, row) in matrix[..l - 1].iter().enumerate() {
            let next_row_it = matrix[i + 1][1..].iter();
            if row.iter().zip(next_row_it).any(|(x, y)| *x != *y) {
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
        assert!(Solution::is_toeplitz_matrix(vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![9, 5, 1, 2],
        ]));
    }

    #[test]
    fn ut_false() {
        // assert!(!Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]));
        // assert!(!Solution::is_toeplitz_matrix(vec![vec![18], vec![66]]));
        assert!(!Solution::is_toeplitz_matrix(vec![
            vec![97, 97],
            vec![80, 6],
            vec![10, 79],
        ]));
    }
}
