pub struct Solution {}

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words.iter() {
            let mut c = word.chars();
            let mut is_palindrome = true;
            while let (Some(start), Some(end)) = (c.next(), c.next_back()) {
                if start != end {
                    is_palindrome = false;
                    break;
                };
            }
            if is_palindrome {
                return word.to_owned();
            }
        }
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::first_palindrome(vec![
                "abc".to_owned(),
                "car".to_owned(),
                "ada".to_owned(),
                "racecar".to_owned(),
                "cool".to_owned(),
            ]),
            String::from("ada")
        );
        assert_eq!(
            Solution::first_palindrome(vec!["notapalindrome".to_owned(), "racecar".to_owned()]),
            String::from("racecar")
        );
        assert_eq!(
            Solution::first_palindrome(vec!["def".to_owned(), "ghi".to_owned()]),
            String::from("")
        );
    }
}
