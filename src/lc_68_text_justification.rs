pub struct Solution {}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut result = vec![];
        let mut line: Vec<String> = vec![];

        for word in words.iter() {
            let word_len = word.len();
            let line_len = line.iter().fold(0, |mut acc, word| {
                acc += word.len();
                acc
            });

            if line_len + word_len + line.len() - 1 < max_width as usize {
                line.push(word.to_owned());
                continue;
            }

            let space_between_words = line.len() - 1;

            if space_between_words == 0 {
                let padding = max_width as usize - line.first().unwrap().len();
                result.push(format!(
                    "{:<width$}",
                    line.first().unwrap(),
                    width = padding
                ));
            } else {
                let mut padded_line = String::from("");
                let extra_padding = max_width as usize - line_len - space_between_words;
                let even_space = extra_padding / space_between_words;
                let mut odd_space = extra_padding % space_between_words;

                for (i, w) in line.iter().enumerate() {
                    if space_between_words == i {
                        padded_line += w;
                        continue;
                    }
                    let mut padded_line = w.to_owned();
                    for _ in 0..=(even_space + 1) {
                        padded_line.push(' ');
                    }
                    if odd_space > 0 {
                        padded_line.push(' ');
                        odd_space -= 1;
                    }
                }
                result.push(padded_line);
            }

            line.clear();
            line.push(word.to_owned());
        }
        let last_line = if line.is_empty() {
            let last_result = result.pop().unwrap();
            last_result
                .split(' ')
                .filter(|x| *x != " ")
                .fold(String::from(""), |mut acc, x| {
                    acc = format!("{acc} {x}");
                    acc
                })
                .trim()
                .to_owned()
        } else {
            line.iter()
                .fold(String::from(""), |mut acc, x| {
                    acc = format!("{acc} {x}");
                    acc
                })
                .trim()
                .to_owned()
        };

        let last_line_len = last_line.len();
        let padded_last_line = format!(
            "{:<width$}",
            last_line,
            width = max_width as usize - last_line_len
        );
        result.push(padded_last_line);

        result
    }
}
