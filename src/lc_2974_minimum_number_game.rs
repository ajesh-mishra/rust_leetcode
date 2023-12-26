pub struct Solution {}

impl Solution {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        (0..nums.len())
            .filter(|&x| x % 2 == 0)
            .for_each(|i| nums.swap(i, i + 1));
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::number_game(vec![5, 4, 2, 3]), vec![3, 2, 5, 4]);
        assert_eq!(Solution::number_game(vec![2, 5]), vec![5, 2]);
    }
}
