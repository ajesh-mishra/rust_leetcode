pub struct Solution {}

impl Solution {
    pub fn sum_of_multiples_1(n: i32) -> i32 {
        let mut result = 0;
        for num in 2..=n {
            if num % 3 == 0 || num % 5 == 0 || num % 7 == 0 {
                result += num;
            }
        }
        result
    }
    pub fn sum_of_multiples_2(n: i32) -> i32 {
        (2..=n)
            .filter(|&num| num % 3 == 0 || num % 5 == 0 || num % 7 == 0)
            .fold(0, |mut acc, num| {
                acc += num;
                acc
            })
    }
}

pub enum Implementations {
    Normal(fn(i32) -> i32),
    Functional(fn(i32) -> i32),
}



#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_small() {
        assert_eq!(Solution::sum_of_multiples_1(7), 21);
        assert_eq!(Solution::sum_of_multiples_1(10), 40);
        assert_eq!(Solution::sum_of_multiples_1(9), 30);
    }
}
