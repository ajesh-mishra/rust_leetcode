use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        fn dfs(
            i: usize,
            s1: String,
            j: usize,
            s2: String,
            s3: String,
            dp: &mut HashMap<(usize, usize), bool>,
        ) -> bool {
            if i == s1.len() && j == s2.len() {
                return true;
            }
            if let Some(val) = dp.get(&(i, j)) {
                return *val;
            }

            let c1 = s1
                .char_indices()
                .filter(|(index, _)| *index == i)
                .map(|(_, x)| x)
                .collect::<String>();
            let c2 = s2
                .char_indices()
                .filter(|(index, _)| *index == j)
                .map(|(_, x)| x)
                .collect::<String>();
            let c3 = s3
                .char_indices()
                .filter(|(index, _)| *index == i + j)
                .map(|(_, x)| x)
                .collect::<String>();
            // let c2 = s2.get(j..j + 1).unwrap();
            // let c3 = s3.get(i + j..i + j + 1).unwrap();

            if i < s1.len()
                && c1 == c3
                && dfs(i + 1, s1.to_owned(), j, s2.to_owned(), s3.to_owned(), dp)
            {
                return true;
            }

            if j < s2.len()
                && c2 == c3
                && dfs(i, s1.to_owned(), j + 1, s2.to_owned(), s3.to_owned(), dp)
            {
                return true;
            }

            dp.entry((i, j)).and_modify(|x| *x = false).or_insert(false);
            false
        }

        dfs(0, s1, 0, s2, s3, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        assert!(Solution::is_interleave(
            "aabcc".to_owned(),
            "dbbca".to_owned(),
            "aadbbcbcac".to_owned()
        ));
    }
}
