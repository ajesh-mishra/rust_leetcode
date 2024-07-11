pub struct Solution {}

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        if !s.contains('(') {
            return s;
        }

        let (mut start, mut end) = (0, 0);

        for (index, c) in s.char_indices() {
            if c == '(' {
                start = index;
            }
            if c == ')' {
                end = index;
                return Solution::reverse_parentheses(format!(
                    "{}{}{}",
                    s.get(..start).unwrap(),
                    s.get(start + 1..end)
                        .unwrap()
                        .chars()
                        .rev()
                        .collect::<String>(),
                    s.get(end + 1..).unwrap()
                ));
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::reverse_parentheses("(abcd)".to_string()),
            "dcba".to_string()
        );
        assert_eq!(
            Solution::reverse_parentheses("(u(love)i)".to_string()),
            "iloveu".to_string()
        );
        assert_eq!(
            Solution::reverse_parentheses("(ed(et(oc))el)".to_string()),
            "leetcode".to_string()
        );
    }
}
