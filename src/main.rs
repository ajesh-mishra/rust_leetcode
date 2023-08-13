use one_leetcode::format_title::convert;
use one_leetcode::lc_242_valid_anagram::Solution as lc_242;

fn main() {
    println!("{}", convert("242. Valid Anagram"));
    println!("{:?}", lc_242::is_anagram(String::from("anagram"), String::from("nagaram")));
}
