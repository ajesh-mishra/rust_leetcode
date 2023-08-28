use one_leetcode::format_title::convert;
use one_leetcode::lc_404_sum_of_left_leaves::Solution as lc;

fn main() {
    println!("{}", convert("404. Sum of Left Leaves"));
    println!(
        "{:#?}",
        lc::sum_of_left_leaves(None)
    );
}
