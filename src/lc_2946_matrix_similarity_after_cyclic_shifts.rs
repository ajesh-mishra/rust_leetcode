pub struct Solution {}

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let mut mat_1: Vec<Vec<i32>> = mat.clone();

        let m = mat.len();
        let n = mat[0].len();

        for _ in 0..k {
            for index in 0..m {
                if index % 2 == 0 {
                    mat_1[index] = mat_1[index]
                        .iter()
                        .cycle()
                        .skip(n - 1)
                        .take(n)
                        .map(|&x| x)
                        .collect::<Vec<i32>>();
                    dbg!(&mat_1[index]);
                } else {
                    mat_1[index] = mat_1[index]
                        .iter()
                        .cycle()
                        .skip(1)
                        .take(n)
                        .map(|&x| x)
                        .collect::<Vec<i32>>();
                    dbg!(&mat_1[index]);
                }
            }
        }

        mat == mat_1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_true() {
        assert!(Solution::are_similar(
            vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]],
            2
        ));
        assert!(Solution::are_similar(vec![vec![2, 2], vec![2, 2]], 3));
    }
    #[test]
    fn ut_false() {
        assert!(!Solution::are_similar(vec![vec![1, 2]], 1))
    }
}
