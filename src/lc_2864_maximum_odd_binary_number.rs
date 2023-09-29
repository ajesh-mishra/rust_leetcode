pub struct Solution {}

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let len = s.len();
        let ones = s.chars().filter(|&x| x == '1').count();
        let result_1 = (1..ones).map(|_| '1').collect::<String>();
        let result_2 = (ones..len).map(|_| '0').collect::<String>();
        format!("{result_1}{result_2}1")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::maximum_odd_binary_number("010".to_owned()),
            String::from("001")
        );
        assert_eq!(
            Solution::maximum_odd_binary_number("0101".to_owned()),
            String::from("1001")
        );
    }
}
