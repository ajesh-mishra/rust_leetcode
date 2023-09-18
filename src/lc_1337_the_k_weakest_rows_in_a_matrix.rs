pub struct Solution {}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut strength_mat: Vec<(i32, i32)> =
            mat.iter()
                .enumerate()
                .fold(vec![], |mut acc, (index, row)| {
                    let strength = row.iter().filter(|&&x| x == 1).count() as i32;
                    acc.push((index as i32, strength));
                    acc
                });
        strength_mat.sort_by(|&a, &b| (a.1).cmp(&b.1));
        strength_mat
            .iter()
            .map(|(r, _)| *r)
            .take(k as usize)
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::k_weakest_rows(
                vec![
                    vec![1, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![1, 0, 0, 0],
                    vec![1, 0, 0, 0]
                ],
                2
            ),
            vec![0, 2]
        );
        assert_eq!(
            Solution::k_weakest_rows(
                vec![
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1]
                ],
                3
            ),
            vec![2, 0, 3]
        )
    }
}
