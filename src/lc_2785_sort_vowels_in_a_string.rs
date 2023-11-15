pub struct Solution {}

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowels = vec![];
        let mut result = String::from("");
        for c in s.chars() {
            match c {
                'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowels.push(c);
                    result.push('_')
                }
                _ => result.push(c),
            }
        }
        vowels.sort();
        let mut v = vowels.iter();
        let mut final_result = String::from("");
        for r in result.chars() {
            if r == '_' {
                final_result.push(*v.next().unwrap());
                continue;
            }
            final_result.push(r);
        }
        final_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::sort_vowels(String::from("lEetcOde")),
            String::from("lEOtcede")
        );
        assert_eq!(
            Solution::sort_vowels(String::from("lYmpH")),
            String::from("lYmpH")
        );
    }
}
