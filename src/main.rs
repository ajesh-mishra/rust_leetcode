use one_leetcode::format_title::convert;
use one_leetcode::lc_1022_sum_of_root_to_leaf_binary_numbers::Solution as lc;

fn main() {
    println!("{}", convert("1022. Sum of Root To Leaf Binary Numbers"));
    println!("{:#?}", lc::sum_root_to_leaf(None));
}
