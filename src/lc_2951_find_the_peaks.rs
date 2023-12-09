pub struct Solution {}

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        mountain
            .windows(3)
            .enumerate()
            .filter(|(_, w)| w[0] < w[1] && w[1] > w[2])
            .map(|(index, _)| index as i32 + 1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(Solution::find_peaks(vec![2, 4, 4]), vec![]);
        assert_eq!(Solution::find_peaks(vec![1, 4, 3, 8, 5]), vec![1, 3]);
    }
}
