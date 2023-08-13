pub fn convert(title: &str) -> String {
    let mut result = String::from("lc_");
    for c in title.trim().chars() {
        match c {
            '.' => {}
            ' ' => result.push('_'),
            'A'..='Z' => result.push_str(c.to_lowercase().to_string().as_str()),
            _ => result.push(c),
        }
    }
    result
}

pub fn convert_2(title: &str) -> String {
    let title = format!("lc_{}", title.trim());
    String::from_utf8(
        title
            .chars()
            .map(|c| match c {
                '.' => None,
                ' ' => Some('_'),
                'A'..='Z' => Some(c.to_lowercase().next().unwrap()),
                _ => Some(c),
            })
            .filter_map(|x| x)
            .map(|c| c as u8)
            .collect::<Vec<u8>>(),
    )
    .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(convert_2("2652. Sum Multiples"), "lc_2652_sum_multiples");
    }
}
