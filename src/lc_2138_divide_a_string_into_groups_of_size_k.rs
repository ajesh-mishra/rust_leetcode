pub struct Solution {}

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut result = s
            .chars()
            .collect::<Vec<char>>()
            .chunks(k as usize)
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<String>>();

        let diff = k as usize - result.last().unwrap().len();

        if diff != 0 {
            let left = result.pop().unwrap();
            let right = format!("{}", fill).repeat(diff);
            result.push(format!("{}{}", left, right));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::divide_string("abcdefghi".to_string(), 3, 'x'),
            vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()]
        );
        assert_eq!(
            Solution::divide_string("abcdefghij".to_string(), 3, 'x'),
            vec![
                "abc".to_owned(),
                "def".to_owned(),
                "ghi".to_owned(),
                "jxx".to_owned(),
            ]
        );
    }
}
