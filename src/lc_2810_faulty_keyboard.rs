pub struct Solution {}

impl Solution {
    pub fn final_string_1(s: String) -> String {
        let mut result = String::from("");
        for c in s.chars() {
            match c {
                'i' => result = result.chars().rev().collect::<String>(),
                _ => result.push(c),
            }
        }
        result
    }
    pub fn final_string(s: String) -> String {
        s.chars().fold(String::from(""), |mut acc, c| {
            match c {
                'i' => acc = acc.chars().rev().collect::<String>(),
                _ => acc.push(c),
            }
            acc
        })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::final_string(String::from("poiinter")),
            String::from("ponter")
        );
    }
}
