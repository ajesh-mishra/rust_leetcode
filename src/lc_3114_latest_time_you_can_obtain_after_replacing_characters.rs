pub struct Solution {}

impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut time = s.split(':');

        let mut hour = time.next().unwrap().chars();
        let new_hour = if let (Some(a), Some(b)) = (hour.next(), hour.next()) {
            match (a, b) {
                ('?', '?') => format!("11"),
                ('0', '?') => format!("09"),
                ('1', '?') => format!("11"),
                ('?', a) if a == '1' || a == '0' => format!("1{a}"),
                ('?', b) => format!("0{b}"),
                (x, y) => format!("{x}{y}"),
            }
        } else {
            unreachable!();
        };

        let mut minute = time.next().unwrap().chars();
        let new_minute = if let (Some(a), Some(b)) = (minute.next(), minute.next()) {
            match (a, b) {
                ('?', '?') => format!("59"),
                (x, '?') => format!("{x}9"),
                ('?', x) => format!("5{x}"),
                (x, y) => format!("{x}{y}"),
            }
        } else {
            unreachable!();
        };

        format!("{}:{}", new_hour, new_minute)
    }
}
