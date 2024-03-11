pub struct Solution {}

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut order_map = vec![0; 26];
        let mut un_order_map = vec![0; 26];

        for c in s.chars() {
            if order.contains(c) {
                order_map[(c as u8 - 97) as usize] += 1;
            } else {
                un_order_map[(c as u8 - 97) as usize] += 1;
            }
        }

        let x = order
            .chars()
            .map(|my_char| {
                format!("{}", my_char).repeat(order_map[(my_char as u8 - 97) as usize] as usize)
            })
            .collect::<String>();

        let y = un_order_map
            .iter()
            .enumerate()
            .filter(|(_, &count)| count > 0)
            .map(|(index, &count)| {
                let my_char = char::from((index + 97) as u8);
                format!("{}", my_char).repeat(count as usize)
            })
            .collect::<String>();

        format!("{}{}", x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::custom_sort_string("cba".to_string(), "abcd".to_string()),
            "cbad".to_string()
        );
        assert_eq!(
            Solution::custom_sort_string("bcafg".to_string(), "abcd".to_string()),
            "bcad".to_string()
        );
    }
}
