pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut cache = vec![vec![0; n + 1]; m + 1];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i == m - 1 && j == n - 1 {
                    cache[i][j] = 1;
                    continue;
                }
                cache[i][j] = cache[i][j + 1] + cache[i + 1][j];
            }
        }
        cache[0][0]
    }
}
