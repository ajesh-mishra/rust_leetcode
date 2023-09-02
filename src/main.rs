use one_leetcode::format_title::convert;
use one_leetcode::lc_2707_extra_characters_in_a_string::Solution as lc;

fn main() {
    println!("{}", convert("2707. Extra Characters in a String"));
    println!(
        "{:#?}",
        lc::min_extra_char(
            "leetscode".to_owned(),
            vec!["leet".to_owned(), "code".to_owned(), "leetcode".to_owned()]
        )
    );
}
