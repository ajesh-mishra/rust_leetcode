use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut max_gold = 0;

        fn dfs(
            i: usize,
            j: usize,
            visited: &mut HashSet<(usize, usize)>,
            max_gold: &mut i32,
            mut gold: i32,
            grid: &Vec<Vec<i32>>,
            m: usize,
            n: usize,
        ) {
            gold += grid[i][j];
            visited.insert((i, j));

            let i = i as i32;
            let j = j as i32;

            for (new_i, new_j) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if new_i < 0 || new_i >= m as i32 {
                    continue;
                }
                if new_j < 0 || new_j >= n as i32 {
                    continue;
                }
                if grid[new_i as usize][new_j as usize] == 0 {
                    continue;
                }
                if visited.contains(&(new_i as usize, new_j as usize)) {
                    continue;
                }
                dfs(
                    new_i as usize,
                    new_j as usize,
                    visited,
                    max_gold,
                    gold,
                    grid,
                    m,
                    n,
                );
                visited.remove(&(new_i as usize, new_j as usize));
            }

            *max_gold = gold.max(*max_gold);
        }

        for i in 0..m {
            for j in 0..n {
                let mut visited = HashSet::new();
                dfs(i, j, &mut visited, &mut max_gold, 0, &grid, m, n)
            }
        }

        max_gold
    }
}
