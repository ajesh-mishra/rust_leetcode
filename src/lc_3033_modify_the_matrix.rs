pub struct Solution {}

impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut answer = matrix.clone();

        for column in 0..n {
            let mut col_max = 0;
            let mut minus_ones = vec![];

            for row in 0..m {
                if answer[row][column] == -1 {
                    minus_ones.push(row);
                }
                col_max = col_max.max(answer[row][column]);
            }

            for minus_one in minus_ones {
                answer[minus_one][column] = col_max;
            }
        }

        answer
    }
}
