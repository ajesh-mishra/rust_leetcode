use one_leetcode::format_title::convert;
use one_leetcode::lc_1337_the_k_weakest_rows_in_a_matrix::Solution as lc;

fn main() {
    println!("{}", convert("1337. The K Weakest Rows in a Matrix"));
    println!(
        "{:#?}",
        lc::k_weakest_rows(
            vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1]
            ],
            3
        )
    );
}
