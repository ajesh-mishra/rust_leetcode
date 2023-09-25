use one_leetcode::format_title::convert;
use one_leetcode::lc_1048_longest_string_chain::Solution as lc;

fn main() {
    println!("{}", convert("389. Find the Difference"));
    println!(
        "{:#?}",
        lc::find_the_difference(vec!["abcd".to_owned(), "abcde".to_owned()])
    );
}
