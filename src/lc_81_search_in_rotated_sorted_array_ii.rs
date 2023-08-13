use std::cmp::Ordering;

pub struct Solution {}

#[derive(Clone, Copy)]
enum StartingFrom {
    Start,
    End,
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let first_element = nums.first().unwrap();
        let starting_from = match target.cmp(first_element) {
            Ordering::Less => StartingFrom::End,
            Ordering::Greater => StartingFrom::Start,
            Ordering::Equal => return true,
        };
        let starting_point: Box<dyn Iterator<Item = &i32>> = match starting_from {
            StartingFrom::Start => Box::new(nums.iter()),
            StartingFrom::End => Box::new(nums.iter().rev()),
        };
        for num in starting_point {
            match (target.cmp(num), starting_from) {
                (Ordering::Equal, _) => return true,
                (Ordering::Greater, StartingFrom::End) => break,
                (Ordering::Less, StartingFrom::Start) => break,
                (_, _) => {}
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        assert!(Solution::search(vec![1, 3, 5], 3));
        assert!(Solution::search(vec![1, 3], 3));
    }
}
