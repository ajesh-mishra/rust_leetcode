use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result_map = HashMap::new();

        nums1.iter().chain(nums2.iter()).for_each(|pair| {
            result_map
                .entry(pair[0])
                .and_modify(|x| *x += pair[1])
                .or_insert(pair[1]);
        });

        let mut result = result_map
            .iter()
            .map(|(&key, &value)| vec![key, value])
            .collect::<Vec<Vec<i32>>>();

        result.sort_by(|a, b| a[0].cmp(&b[0]));

        result
    }
    pub fn merge_arrays_1(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        let mut nums1_iter = nums1.iter();
        let mut nums2_iter = nums2.iter();

        let mut n1_next = nums1_iter.next();
        let mut n2_next = nums2_iter.next();

        while let (Some(n1), Some(n2)) = (n1_next, n2_next) {
            match n1[0].cmp(&n2[0]) {
                Ordering::Equal => {
                    result.push(vec![n1[0], n1[1] + n2[1]]);
                    n1_next = nums1_iter.next();
                    n2_next = nums2_iter.next();
                }
                Ordering::Greater => {
                    result.push(n2.clone());
                    n2_next = nums2_iter.next();
                }
                Ordering::Less => {
                    result.push(n1.clone());
                    n1_next = nums1_iter.next();
                }
            }
        }

        if let Some(n) = n1_next {
            result.push(n.clone());
        }

        if let Some(n) = n2_next {
            result.push(n.clone());
        }

        while let Some(n) = nums1_iter.next() {
            result.push(n.clone());
        }

        while let Some(n) = nums2_iter.next() {
            result.push(n.clone());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::merge_arrays(
                vec![vec![1, 2], vec![2, 3], vec![4, 5]],
                vec![vec![1, 4], vec![3, 2], vec![4, 1]]
            ),
            vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]]
        );
        assert_eq!(
            Solution::merge_arrays(
                vec![vec![2, 4], vec![3, 6], vec![5, 5]],
                vec![vec![1, 3], vec![4, 3]]
            ),
            vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]]
        );
    }
}
