use one_leetcode::format_title::convert;
use one_leetcode::lc_999_available_captures_for_rook::Solution as lc;

fn main() {
    println!("{}", convert("999. Available Captures for Rook"));
    println!(
        "{:#?}",
        lc::num_rook_captures(vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.']
        ])
    );
}
