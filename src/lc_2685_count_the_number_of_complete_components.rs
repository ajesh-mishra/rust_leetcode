use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    fn create_adjacency_list(n: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adjacency_list = vec![vec![]; n];
        for edge in edges {
            if let [source, destination] = edge.iter().as_slice() {
                adjacency_list[*source as usize].push(*destination);
                adjacency_list[*destination as usize].push(*source);
            } else {
                unreachable!();
            }
        }
        adjacency_list
    }

    fn is_complete(components: &mut Vec<i32>, adjacency_list: &Vec<Vec<i32>>) -> bool {
        for &component in components.iter() {
            if components.len() - 1 != adjacency_list.get(component as usize).unwrap().len() {
                return false;
            }
        }
        true
    }

    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let adjacency_list = Solution::create_adjacency_list(n as usize, &edges);
        fn dfs(
            node: i32,
            adjacency_list: &Vec<Vec<i32>>,
            visited: &mut HashSet<i32>,
            components: &mut Vec<i32>,
        ) -> () {
            if visited.contains(&node) {
                return ();
            }

            visited.insert(node);
            components.push(node);

            for &n in adjacency_list.get(node as usize).unwrap() {
                dfs(n, adjacency_list, visited, components)
            }
        }

        let mut visited: HashSet<i32> = HashSet::new();
        let mut count = 0;
        let mut components = vec![];

        for node in 0..n {
            if visited.contains(&node) {
                continue;
            }
            dfs(node, &adjacency_list, &mut visited, &mut components);
            if Solution::is_complete(&mut components, &adjacency_list) {
                count += 1;
            }
            components.clear();
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
            Solution::count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]],
            ),
            3
        );
        assert_eq!(
            Solution::count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]],
            ),
            1
        );
    }
}
