use one_leetcode::format_title::convert;
use one_leetcode::lc_392_is_subsequence::Solution as lc;

fn main() {
    println!("{}", convert("392. Is Subsequence"));
    println!(
        "{:#?}",
        lc::maximum_number_of_string_pairs(vec![
            "cd".to_owned(),
            "ac".to_owned(),
            "dc".to_owned(),
            "ca".to_owned(),
            "zz".to_owned()
        ])
    );
}
