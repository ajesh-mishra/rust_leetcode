use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = land.len();
        let n = land[0].len();

        fn dfs(
            i: usize,
            j: usize,
            land: &Vec<Vec<i32>>,
            group: &mut [usize; 4],
            visited: &mut HashSet<(usize, usize)>,
            m: i32,
            n: i32,
        ) {
            visited.insert((i, j));

            if i < group[0] {
                (group[0], group[1]) = (i, j);
            }
            if i == group[0] && j < group[1] {
                (group[0], group[1]) = (i, j);
            }
            if i >= group[2] {
                (group[2], group[3]) = (i, j);
            }

            let i = i as i32;
            let j = j as i32;

            for (new_i, new_j) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if new_i < 0 || new_i >= m {
                    continue;
                }
                if new_j < 0 || new_j >= n {
                    continue;
                }
                if land[new_i as usize][new_j as usize] == 0 {
                    continue;
                }
                if visited.contains(&(new_i as usize, new_j as usize)) {
                    continue;
                }
                dfs(new_i as usize, new_j as usize, land, group, visited, m, n);
            }
        }

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut result = vec![];

        for x in 0..m {
            for y in 0..n {
                if visited.contains(&(x, y)) || land[x][y] == 0 {
                    continue;
                }
                let mut group = [m - 1, n - 1, 0, 0];
                dfs(x, y, &land, &mut group, &mut visited, m as i32, n as i32);
                result.push(group.iter().map(|&x| x as i32).collect::<Vec<i32>>());
            }
        }

        result
    }
}
