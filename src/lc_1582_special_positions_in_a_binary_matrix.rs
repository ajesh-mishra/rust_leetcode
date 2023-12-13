pub struct Solution {}

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let (mut count, m, n) = (0, mat.len(), mat[0].len());
        for x in 0..m {
            for y in 0..n {
                if mat[x][y] != 1 {
                    continue;
                }
                let vertical = (0..m).map(|i| mat[i][y]).sum::<i32>();
                let horizontal = (0..n).map(|j| mat[x][j]).sum::<i32>();
                if vertical + horizontal == 2 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            1
        );
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }
}
