pub struct Solution {}

impl Solution {
    pub fn hardest_worker(_n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut min_id = i32::MAX;
        let mut max_time = 0;
        let mut prev_time = 0;
        for log in logs.iter() {
            let id = *log.first().unwrap();
            let time = *log.last().unwrap() - prev_time;
            prev_time += time;
            if max_time < time {
                max_time = time;
                min_id = id;
            }
            if max_time == time && min_id > id {
                min_id = id;
            }
        }
        min_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::hardest_worker(10, vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]]),
            1
        );
        assert_eq!(
            Solution::hardest_worker(26, vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]]),
            3
        );
    }
}
