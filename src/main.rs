use one_leetcode::format_title::convert;
use one_leetcode::lc_2682_find_the_losers_of_the_circular_game::Solution as lc;

fn main() {
    println!("{}", convert("2682. Find the Losers of the Circular Game"));
    println!(
        "{:#?}",
        lc::circular_game_losers(5, 2)
    );
}
