use one_leetcode::format_title::convert;
use one_leetcode::lc_459_repeated_substring_pattern::Solution as lc;

fn main() {
    println!("{}", convert("459. Repeated Substring Pattern"));
    println!(
        "{:#?}",
        lc::repeated_substring_pattern("aba".to_string())
    );
}
    