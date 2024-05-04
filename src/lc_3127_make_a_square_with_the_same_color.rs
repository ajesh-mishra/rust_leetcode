use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        fn dfs(
            i: usize,
            j: usize,
            color: char,
            visited: &mut HashSet<(usize, usize)>,
            path: &mut Vec<(usize, usize)>,
            grid: &Vec<Vec<char>>,
        ) {
            visited.insert((i, j));
            path.push((i, j));
            let mut neighbours = vec![];

            if let Some(x) = i.checked_add(1) {
                neighbours.push((x, j));
            }
            if let Some(x) = i.checked_sub(1) {
                neighbours.push((x, j));
            }
            if let Some(x) = j.checked_add(1) {
                neighbours.push((i, x));
            }
            if let Some(x) = j.checked_sub(1) {
                neighbours.push((i, x));
            }

            for (new_i, new_j) in neighbours.iter().copied() {
                if new_i >= 3 || new_j >= 3 {
                    continue;
                }
                if visited.contains(&(new_i, new_j)) {
                    continue;
                }
                if grid[new_i][new_j] != color {
                    continue;
                }
                dfs(new_i, new_j, color, visited, path, grid);
            }
        }

        let mut visited = HashSet::new();

        for i in 0..3 {
            for j in 0..3 {
                if visited.contains(&(i, j)) {
                    continue;
                }

                let mut path = vec![];
                dfs(i, j, grid[i][j], &mut visited, &mut path, &grid);

                if path.len() > 3 {
                    return true;
                }

                if path.len() == 3 {
                    let x = path
                        .iter()
                        .map(|(_, x)| x)
                        .collect::<HashSet<&usize>>()
                        .len();
                    let y = path
                        .iter()
                        .map(|(x, _)| x)
                        .collect::<HashSet<&usize>>()
                        .len();
                    if x == 1 || y == 1 {
                        return true;
                    }
                }
            }
        }

        false
    }
}
