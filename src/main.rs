use one_leetcode::format_title::convert;
use one_leetcode::lc_1615_maximal_network_rank::Solution as lc;

fn main() {
    println!("{}", convert("1615. Maximal Network Rank"));
    println!(
        "{:#?}",
        lc::maximal_network_rank(4, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]])
    );
}
