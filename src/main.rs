use one_leetcode::format_title::convert;
use one_leetcode::lc_48_rotate_image::Solution as lc;

fn main() {
    println!("{}", convert("48. Rotate Image"));
    println!(
        "{:#?}",
        lc::rotate(&mut vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
}
    