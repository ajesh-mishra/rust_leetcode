use one_leetcode::format_title::convert;
use one_leetcode::lc_2828_check_if_a_string_is_an_acronym_of_words::Solution as lc;

fn main() {
    println!(
        "{}",
        convert("2828. Check if a String Is an Acronym of Words")
    );
    println!(
        "{:#?}",
        lc::is_acronym(
            vec!["alice".to_owned(), "bob".to_owned(), "charlie".to_owned()],
            "abc".to_owned()
        )
    );
}
