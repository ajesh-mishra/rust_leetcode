pub struct RecentCounter {
    count: Vec<i32>,
    window: i32
}

impl RecentCounter {

    fn new() -> Self {
        RecentCounter { count: vec![], window: 3000 }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.count.push(t);
        for (index, time) in self.count.iter().rev().enumerate() {
            if *time < t - self.window {
                return index as i32;
            }
        }

        return self.count.len() as i32;
    }
}

#[cfg(test)]
mod test {
    use super::RecentCounter;

    #[test]
    fn ut() {
        let mut rc = RecentCounter::new();
        rc.ping(10);
    }
}

