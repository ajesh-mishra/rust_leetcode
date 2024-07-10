pub struct Solution {}

impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        (0..s.len())
            .map(|index| {
                let pos = (index + k as usize) % s.len();
                s.get(pos..pos + 1).unwrap()
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::get_encrypted_string("dart".to_owned(), 3),
            "tdar".to_owned()
        );
        assert_eq!(
            Solution::get_encrypted_string("aaa".to_owned(), 1),
            "aaa".to_owned()
        );
    }
}
