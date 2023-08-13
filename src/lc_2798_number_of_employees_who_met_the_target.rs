pub struct Solution {}

impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours
            .iter()
            .filter(|&&hour| hour >= target)
            .count() as _
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_one() {
        assert_eq!(
            Solution::number_of_employees_who_met_target(vec![0,1,2,3,4], 2),
            3
        )
    }
}