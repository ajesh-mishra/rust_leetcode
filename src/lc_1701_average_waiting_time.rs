pub struct Solution {}

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let (mut total_waiting_time, mut prev_time) = (0.0, 0);

        for time in &customers {
            let st = prev_time.max(time[0]);
            prev_time = st + time[1];

            if time[0] < prev_time {
                total_waiting_time += (prev_time - time[0]) as f64;
            }
        }

        total_waiting_time / customers.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]),
            5.00000
        );
        assert_eq!(
            Solution::average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]]),
            3.25000
        );
    }
}
