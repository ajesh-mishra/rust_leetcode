use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

pub struct Solution {}

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let words = HashSet::from_iter(dictionary);
        let mut cache: HashMap<usize, i32> = HashMap::from_iter([(s.len(), 0)]);

        fn dp(s: &str, dictionary: &HashSet<String>, i: usize, cache: &mut HashMap<usize, i32>) -> i32 {
            if i == s.len() {
                return 0;
            }
            if let Some(res) = cache.get(&i) {
                return *res;
            }
            let mut r = 1 + dp(s, dictionary, i + 1, cache);
            for j in i..s.len() {
                let str_slice: &String = &format!("{}", s.get(i..=j).unwrap());
                if dictionary.contains(str_slice) {
                    r = r.min(dp(s, dictionary, j + 1, cache));
                }
            }
            cache.entry(i).or_insert(r);
            return r;
        }

        dp(s.as_str(), &words, 0, &mut cache)
    }
}
