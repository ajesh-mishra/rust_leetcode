use one_leetcode::format_title::convert;
use one_leetcode::lc_542_01_matrix::Solution as lc;

fn main() {
    println!("{}", convert("542. 01 Matrix"));
    println!(
        "{:#?}",
        lc::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])
    );
}
