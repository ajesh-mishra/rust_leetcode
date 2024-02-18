pub struct Solution {}

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut row_count = vec![0; m as usize];
        let mut col_count = vec![0; n as usize];

        for item in indices.iter() {
            if let [i, j] = item.as_slice() {
                row_count[*i as usize] += 1;
                col_count[*j as usize] += 1;
            }
        }

        let result = row_count
            .iter()
            .map(|&x| vec![x; n as usize])
            .collect::<Vec<Vec<i32>>>();

        let mut odd_count = 0;

        for (j, &count) in col_count.iter().enumerate() {
            for i in 0..m as usize {
                if (result[i][j] + count) % 2 != 0 {
                    odd_count += 1;
                }
            }
        }

        odd_count
    }
}
