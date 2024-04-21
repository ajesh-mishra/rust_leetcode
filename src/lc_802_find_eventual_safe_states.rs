use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(
            node: usize,
            visited: &mut HashSet<usize>,
            safe_nodes: &mut HashSet<usize>,
            graph: &Vec<Vec<i32>>,
        ) -> bool {
            if safe_nodes.contains(&node) {
                return true;
            }
            if visited.contains(&node) {
                return false;
            }
            visited.insert(node);
            for &neighbour in &graph[node] {
                if !dfs(neighbour as usize, visited, safe_nodes, graph) {
                    return false;
                }
            }
            safe_nodes.insert(node);
            true
        }

        let mut visited = HashSet::new();
        let mut safe_nodes = graph
            .iter()
            .enumerate()
            .filter(|(_, x)| x.is_empty())
            .map(|(index, _)| index)
            .collect::<HashSet<usize>>();

        for node in 0..graph.len() {
            if visited.contains(&node) {
                continue;
            }
            dfs(node, &mut visited, &mut safe_nodes, &graph);
        }

        let mut result = safe_nodes.iter().map(|&x| x as i32).collect::<Vec<i32>>();
        result.sort();
        result
    }
}
