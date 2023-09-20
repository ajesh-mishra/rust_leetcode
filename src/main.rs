use one_leetcode::format_title::convert;
use one_leetcode::lc_2859_sum_of_values_at_indices_with_k_set_bits::Solution as lc;

fn main() {
    println!(
        "{}",
        convert("2859. Sum of Values at Indices With K Set Bits")
    );
    println!(
        "{:#?}",
        lc::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1)
    );
}
