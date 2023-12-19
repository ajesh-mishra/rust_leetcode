pub struct Solution {}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = img.len();
        let n = img[0].len();
        let mut result = vec![vec![0_i32; n]; m];

        for i in 0..m {
            for j in 0..n {
                let i_start = i.checked_sub(1).unwrap_or(0);
                let i_end = (i + 1).min(m - 1);
                let j_start = (j).checked_sub(1).unwrap_or(0);
                let j_end = (j + 1).min(n - 1);

                let (mut neighbour_sum, mut count) = (0, 0);
                for a in i_start..=i_end {
                    for b in j_start..=j_end {
                        neighbour_sum += img[a][b];
                        count += 1;
                    }
                }
                result[i][j] = neighbour_sum / count;
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
            Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
        assert_eq!(
            Solution::image_smoother(vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]]),
            vec![vec![137, 141, 137], vec![141, 138, 141], vec![137, 141, 137]]
        );
    }
}
