use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut adjacency_list = vec![vec![]; n as usize];
        let mut visited = HashSet::new();

        for edge in edges.iter() {
            let (&first, &second) = match edge.as_slice() {
                [first, second] => (first, second),
                _ => unreachable!(),
            };
            adjacency_list[first as usize].push(second);
            adjacency_list[second as usize].push(first);
        }

        fn dfs(
            adjacency_list: &Vec<Vec<i32>>,
            source: i32,
            destination: i32,
            visited: &mut HashSet<i32>,
        ) -> bool {
            if source == destination {
                return true;
            }
            visited.insert(source);
            for neighbour in &adjacency_list[source as usize] {
                if visited.contains(neighbour) {
                    continue;
                }
                if dfs(adjacency_list, *neighbour, destination, visited) {
                    return true;
                }
            }
            false
        }

        dfs(&adjacency_list, source, destination, &mut visited)
    }
}
