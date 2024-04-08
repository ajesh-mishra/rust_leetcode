pub struct Solution {}

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        fn remove_parenthesis<T>(chars: T, left: char, right: char) -> (i32, String)
        where
            T: Iterator<Item = char> + DoubleEndedIterator,
        {
            let mut inner_running_value: i32 = 0;
            let mut inner_result = String::from("");

            for c in chars {
                match c {
                    x if x == left => {
                        inner_running_value += 1;
                        inner_result.push(x);
                    }
                    x if x == right => {
                        if inner_running_value - 1 < 0 {
                            continue;
                        }
                        inner_running_value -= 1;
                        inner_result.push(right);
                    }
                    _ => inner_result.push(c),
                }
            }

            (inner_running_value, inner_result)
        }

        let (running_value, result) = remove_parenthesis(s.chars(), '(', ')');

        if running_value != 0 {
            let (_, result) = remove_parenthesis(result.chars().rev(), ')', '(');
            return result.chars().rev().collect::<String>();
        }

        result
    }
}
