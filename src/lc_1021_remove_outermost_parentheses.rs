pub struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut count = 0;
        let mut temp = String::from("");
        let mut decompositions = vec![];

        for (index, char) in s.char_indices() {
            match char {
                '(' => count += 1,
                ')' => count -= 1,
                _ => {}
            }
            temp.push(char);
            if index != 0 && count == 0 {
                decompositions.push(temp);
                temp = String::from("");
            }
        }
        decompositions
            .iter()
            .map(|s| s.chars().skip(1).take(s.len() - 2).collect::<String>())
            .collect::<String>()
    }
}
