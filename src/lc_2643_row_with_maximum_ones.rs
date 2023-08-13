pub struct Solution {}

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut row_number, mut ones) = (0, 0);
        for (index, row) in mat.iter().enumerate() {
            let local_ones = row.iter().filter(|&&x| x == 1).count();
            if index == 0 {
                ones = local_ones;
                continue;
            }
            if local_ones > ones {
                row_number = index;
                ones = local_ones;
            }
        }
        vec![row_number as i32, ones as i32]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            vec![1, 2]
        );
    }
}
