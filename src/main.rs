use one_leetcode::format_title::convert;
use one_leetcode::lc_2864_maximum_odd_binary_number::Solution as lc;

fn main() {
    println!("{}", convert("2605. Form Smallest Number From Two Digit Arrays"));
    println!("{:#?}", lc::maximum_odd_binary_number("bcabc".to_owned()));
}
