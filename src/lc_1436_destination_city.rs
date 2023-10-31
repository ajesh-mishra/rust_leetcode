pub struct Solution {}

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut sources = vec![];
        let mut destinations = vec![];
        for path in paths.iter() {
            sources.push(path.first().unwrap());
            destinations.push(path.last().unwrap());
        }
        for destination in destinations.iter() {
            if !sources.contains(destination) {
                return destination.to_string();
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut() {
        assert_eq!(
            Solution::dest_city(vec![
                vec!["London".to_owned(), "New York".to_owned()],
                vec!["New York".to_owned(), "Lima".to_owned()],
                vec!["Lima".to_owned(), "Sao Paulo".to_owned()]
            ]),
            "Sao Paulo".to_owned()
        )
    }
}
