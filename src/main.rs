use one_leetcode::format_title::convert;
use one_leetcode::lc_62_unique_paths::Solution as lc;

fn main() {
    println!("{}", convert("62. Unique Paths"));
    println!("{:#?}", lc::unique_paths(3, 7));
}
