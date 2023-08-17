use std::collections::VecDeque;
const DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub struct Solution {}

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let length = mat.len();
        let width = mat.first().unwrap().len();

        let mut queue = VecDeque::with_capacity(mat.len());
        let mut visited = vec![vec![false; width]; length];

        for i in 0..length {
            for j in 0..width {
                if mat[i][j] != 0 {
                    continue;
                }
                visited[i][j] = true;
                queue.push_back((i as i32, j as i32, 0));
            }
        }

        while let Some((x, y, count)) = queue.pop_front() {
            for (x_delta, y_delta) in DIRECTIONS {
                let x_new = x + x_delta as i32;
                let y_new = y + y_delta as i32;
                if x_new < 0
                    || y_new < 0
                    || x_new == length as i32
                    || y_new == width as i32
                    || visited[x_new as usize][y_new as usize]
                {
                    continue;
                }
                visited[x_new as usize][y_new as usize] = true;
                mat[x_new as usize][y_new as usize] = count + 1;
                queue.push_back((x_new, y_new, count + 1));
            }
        }

        mat
    }
}
