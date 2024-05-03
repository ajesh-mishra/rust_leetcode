use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn open_lock(mut deadends: Vec<String>, target: String) -> i32 {
        if deadends.contains(&String::from("0000")) {
            return -1;
        }

        let mut queue = VecDeque::from([("0000".to_owned(), 0)]);

        while !queue.is_empty() {
            let (node, depth) = queue.pop_front().unwrap();
            if node == target {
                return depth;
            }
            let node = node.parse::<i32>().unwrap();
            let mut new_nodes = VecDeque::new();
            for place in [1, 10, 100, 1000] {
                match node / (place) % 10 {
                    0 => {
                        new_nodes.push_back((format!("{}", node + 1 * place), depth + 1));
                        new_nodes.push_back((format!("{}", node + 9 * place), depth + 1));
                    }
                    9 => {
                        new_nodes.push_back((format!("{}", node - 1 * place), depth + 1));
                        new_nodes.push_back((format!("{}", node - 9 * place), depth + 1));
                    }
                    _ => {
                        new_nodes.push_back((format!("{}", node - 1 * place), depth + 1));
                        new_nodes.push_back((format!("{}", node + 1 * place), depth + 1));
                    }
                }
            }
            for (n, d) in new_nodes {
                if !deadends.contains(&n) {
                    deadends.push(n.to_owned());
                    queue.push_back((n, d));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::open_lock(
                vec![
                    "0201".to_owned(),
                    "0101".to_owned(),
                    "0102".to_owned(),
                    "1212".to_owned(),
                    "2002".to_owned()
                ],
                "0202".to_owned()
            ),
            6
        );
        // assert_eq!(
        //     Solution::open_lock(vec!["8888".to_owned()], "0009".to_owned()),
        //     1
        // );
        // assert_eq!(
        //     Solution::open_lock(
        //         vec![
        //             "8887".to_owned(),
        //             "8889".to_owned(),
        //             "8878".to_owned(),
        //             "8898".to_owned(),
        //             "8788".to_owned(),
        //             "8988".to_owned(),
        //             "7888".to_owned(),
        //             "9888".to_owned()
        //         ],
        //         "8888".to_owned()
        //     ),
        //     -1
        // );
    }
}
