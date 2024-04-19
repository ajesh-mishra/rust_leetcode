use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid.first().unwrap().len();

        fn dfs(
            x: usize,
            y: usize,
            visited: &mut HashSet<(usize, usize)>,
            grid: &Vec<Vec<char>>,
            m: usize,
            n: usize,
        ) {
            visited.insert((x, y));

            let mut adjacent_points = vec![];
            if let Some(x1) = x.checked_sub(1) {
                adjacent_points.push((x1, y));
            }
            if x + 1 < m {
                adjacent_points.push((x + 1, y));
            }
            if let Some(y1) = y.checked_sub(1) {
                adjacent_points.push((x, y1));
            }
            if y + 1 < n {
                adjacent_points.push((x, y + 1));
            }
            
            for (new_x, new_y) in adjacent_points {
                if grid[new_x][new_y] == '1' && !visited.contains(&(new_x, new_y)) {
                    dfs(new_x, new_y, visited, grid, m, n);
                }
            }
        }

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut count = 0;

        for x in 0..m {
            for y in 0..n {
                if grid[x][y] == '1' && !visited.contains(&(x, y)) {
                    dfs(x, y, &mut visited, &grid, m, n);
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
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
