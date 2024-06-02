pub struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut i, mut j) = (0, s.len());

        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}
