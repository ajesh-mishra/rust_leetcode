use std::cmp::Ordering;
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        nums.sort();
        if let [x, y, z] = nums.iter().as_slice() {
            match (x + y).cmp(z) {
                Ordering::Greater => {}
                _ => return String::from("none"),
            }
        }
        match nums.iter().collect::<HashSet<&i32>>().len() {
            1 => String::from("equilateral"),
            2 => String::from("isosceles"),
            3 => String::from("scalene"),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::triangle_type(vec![3, 3, 3]),
            String::from("equilateral")
        );
        assert_eq!(
            Solution::triangle_type(vec![3, 4, 5]),
            String::from("scalene")
        );
    }
}
