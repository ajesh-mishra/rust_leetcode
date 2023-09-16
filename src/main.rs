use one_leetcode::format_title::convert;
use one_leetcode::lc_2765_longest_alternating_subarray::Solution as lc;

fn main() {
    println!("{}", convert("2765. Longest Alternating Subarray"));
    println!("{:#?}", lc::alternating_subarray(vec![2, 3, 4, 3, 4]));
}
