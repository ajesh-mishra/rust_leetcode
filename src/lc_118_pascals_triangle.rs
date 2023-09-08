pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![1]];

        for row in 1..=num_rows {
            if row == 1 {
                continue;
            }
            if row == 2 {
                result.push(vec![1, 1]);
                continue;
            }

            let last_row = result.last().unwrap();
            let mut new_row = vec![1];

            for value in last_row.windows(2) {
                new_row.push(value[0] + value[1]);
            }
            
            new_row.push(1);
            result.push(new_row);
        }

        result
    }
}

