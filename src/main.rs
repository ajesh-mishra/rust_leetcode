use one_leetcode::format_title::convert;
use one_leetcode::lc_263_ugly_number::Solution as lc;

fn main() {
    println!("{}", convert("263. Ugly Number"));
    println!(
        "{:#?}",
        lc::is_ugly(6)
    );
}
    