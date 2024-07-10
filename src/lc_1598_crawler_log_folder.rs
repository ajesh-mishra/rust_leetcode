pub struct Solution {}

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter().fold(0, |mut acc, log| {
            match log.as_str() {
                "./" => {}
                "../" if acc == 0 => {}
                "../" => acc -= 1,
                _ => acc += 1,
            }
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_owned(),
                "d2/".to_owned(),
                "../".to_owned(),
                "d21/".to_owned(),
                "./".to_owned()
            ]),
            2
        );
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_owned(),
                "d2/".to_owned(),
                "./".to_owned(),
                "d3/".to_owned(),
                "../".to_owned(),
                "d31/".to_owned()
            ]),
            3
        );
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_owned(),
                "../".to_owned(),
                "../".to_owned(),
                "../".to_owned()
            ]),
            0
        );
    }
}
