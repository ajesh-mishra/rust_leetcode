use one_leetcode::format_title::convert;
use one_leetcode::lc_118_pascals_triangle::Solution as lc;

fn main() {
    println!("{}", convert("118. Pascals Triangle"));
    println!("{:#?}", lc::generate(5));
}
