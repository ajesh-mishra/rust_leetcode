pub struct Solution {}

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut diagonal_and_area = dimensions
            .iter()
            .map(|d| {
                let diagonal = ((d[0] * d[0] + d[1] * d[1]) as f64).sqrt();
                let area = d[0] * d[1];
                (diagonal, area)
            })
            .collect::<Vec<(f64, i32)>>();
        diagonal_and_area.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap().then(b.1.cmp(&a.1)));
        dbg!(&diagonal_and_area);
        diagonal_and_area[0].1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::area_of_max_diagonal(vec![vec![9, 3], vec![8, 6]]),
            48
        );
        assert_eq!(
            Solution::area_of_max_diagonal(vec![vec![3, 4], vec![4, 3]]),
            12
        );
        assert_eq!(
            Solution::area_of_max_diagonal(vec![vec![10, 3], vec![5, 9], vec![8, 3]]),
            30
        );
    }
}
