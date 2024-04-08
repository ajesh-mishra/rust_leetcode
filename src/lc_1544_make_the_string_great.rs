pub struct Solution {}

impl Solution {
    pub fn make_good(s: String) -> String {
        fn remove_bad(string: String, length: usize) -> String {
            let (mut i, mut new_string) = (0, String::from(""));

            while i < length {
                if let (Some(f), Some(s)) = (string.chars().nth(i), string.chars().nth(i + 1)) {
                    let first = f as u8;
                    let second = s as u8;
                    
                    if first.abs_diff(second) == 32 {
                        i += 2;
                    } else {
                        new_string.push(f);
                        if i + 1 == length - 1 {
                            new_string.push(s);
                            break;
                        }
                        i += 1;
                    }
                } else {
                    new_string.push(string.chars().nth(i).unwrap());
                    break;
                }
            }

            if new_string.len() == length {
                return new_string;
            }

            remove_bad(new_string.to_string(), new_string.len())
        }
        remove_bad(s.to_string(), s.len())
    }
}
