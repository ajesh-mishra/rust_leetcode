use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = HashSet::new();

        fn dfs(room: i32, rooms: &Vec<Vec<i32>>, visited: &mut HashSet<i32>) {
            if visited.contains(&room) {
                return;
            }
            visited.insert(room);
            for &r in rooms.get(room as usize).unwrap().iter() {
                dfs(r, rooms, visited)
            }
        }

        dfs(0, &rooms, &mut visited);
        visited.len() == rooms.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert!(Solution::can_visit_all_rooms(vec![
            vec![1],
            vec![2],
            vec![3],
            vec![],
        ]));
        assert!(!Solution::can_visit_all_rooms(vec![
            vec![1, 3],
            vec![3, 0, 1],
            vec![2],
            vec![0],
        ]));
    }
}
