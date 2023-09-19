use one_leetcode::format_title::convert;
use one_leetcode::lc_287_find_the_duplicate_number::Solution as lc;

fn main() {
    println!("{}", convert("287. Find the Duplicate Number"));
    println!("{:#?}", lc::find_duplicate(vec![1, 3, 4, 2, 2]));
}
