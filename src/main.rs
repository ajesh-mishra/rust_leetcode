use one_leetcode::format_title::convert;
use one_leetcode::lc_2839_check_if_strings_can_be_made_equal_with_operations_i::Solution as lc;

fn main() {
    println!(
        "{}",
        convert("2839. Check if Strings Can be Made Equal With Operations I")
    );
    println!(
        "{:#?}",
        lc::can_be_equal("abcd".to_owned(), "cdab".to_owned())
    );
}
