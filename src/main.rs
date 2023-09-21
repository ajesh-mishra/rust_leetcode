use one_leetcode::format_title::convert;
use one_leetcode::lc_2744_find_maximum_number_of_string_pairs::Solution as lc;

fn main() {
    println!("{}", convert("2744. Find Maximum Number of String Pairs"));
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
