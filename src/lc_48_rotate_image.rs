pub struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        let mut rotated = vec![vec![0; len]; len];

        for (j, row) in matrix.iter().enumerate() {
            for (i, &value) in row.iter().enumerate() {
                rotated[i][len - j - 1] = value;
            }
        }

        for (i, row) in rotated.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                matrix[i][j] = value;
            }
        }
    }
}