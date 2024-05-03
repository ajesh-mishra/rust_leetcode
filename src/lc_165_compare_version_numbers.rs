pub struct Solution {}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1_iter = version1.split('.');
        let mut v2_iter = version2.split('.');

        let mut v1 = v1_iter.next();
        let mut v2 = v2_iter.next();

        let get_value = |v: Option<&str>| {
            if let Some(x) = v {
                x.parse::<i32>().unwrap()
            } else {
                0
            }
        };

        while v1.is_some() || v2.is_some() {
            let a = get_value(v1);
            let b = get_value(v2);

            if a > b {
                return 1;
            }
            if a < b {
                return -1;
            }

            v1 = v1_iter.next();
            v2 = v2_iter.next();
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::compare_version("1.01".to_owned(), "1.001".to_owned()),
            0
        );
        assert_eq!(
            Solution::compare_version("1.0".to_owned(), "1.0.0".to_owned()),
            0
        );
        assert_eq!(
            Solution::compare_version("0.1".to_owned(), "1.1".to_owned()),
            -1
        );
    }
}
