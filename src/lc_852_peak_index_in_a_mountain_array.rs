pub struct Solution {}

impl Solution {
    pub fn peak_index_in_mountain_array_1(arr: Vec<i32>) -> i32 {
        let mut peak = (0, arr[0]);
        for (index, &value) in arr.iter().enumerate() {
            if value >= peak.1 {
                peak = (index, value);
            } else {
                break;
            }
        }
        peak.0 as _
    }
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        fn inner(start: usize, mid: usize, end: usize, arr: &Vec<i32>) -> i32 {
            match (arr[mid - 1] <= arr[mid], arr[mid] >= arr[mid + 1]) {
                (true, true) => return mid as i32,
                (true, false) => inner(mid, (mid + end) / 2, end, arr),
                (_, _) => inner(start, (start + mid) / 2, mid, arr)
            }

        }
        inner(0, arr.len() / 2, arr.len(), &arr)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_one() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
    }
}
