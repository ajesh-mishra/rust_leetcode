pub struct Solution {}

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut paths = vec![0; n];
        let mut direct = vec![vec![0; n as usize]; n as usize];

        for road in roads.iter() {
            if let [r1, r2] = road.as_slice() {
                let x = *r1 as usize;
                let y = *r2 as usize;
                paths[x] += 1;
                paths[y] += 1;
                direct[x][y] = 1;
                direct[y][x] = 1;       
            }
        }

        let mut network_rank = 0;

        for i in 0..n - 1 {
            for j in i + 1..n {
                let current = paths[i] + paths[j] - direct[i][j];
                network_rank = network_rank.max(current);
            }
        }

        network_rank
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::maximal_network_rank(4, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]]),
            4
        );
        assert_eq!(
            Solution::maximal_network_rank(
                5,
                vec![
                    vec![0, 1],
                    vec![0, 3],
                    vec![1, 2],
                    vec![1, 3],
                    vec![2, 3],
                    vec![2, 4]
                ]
            ),
            5
        );
        assert_eq!(
            Solution::maximal_network_rank(
                8,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![2, 4],
                    vec![5, 6],
                    vec![5, 7]
                ]
            ),
            5
        );
    }
}
