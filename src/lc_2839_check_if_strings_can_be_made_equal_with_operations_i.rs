pub struct Solution {}

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }

        let s2_0 = s2.get(0..1).unwrap();
        let s2_1 = s2.get(1..2).unwrap();
        let s2_2 = s2.get(2..3).unwrap();
        let s2_3 = s2.get(3..4).unwrap();
        
        if s1 == format!("{s2_2}{s2_1}{s2_0}{s2_3}") {
            return true;
        }
        if s1 == format!("{s2_0}{s2_3}{s2_2}{s2_1}") {
            return true;
        }
        if s1 == format!("{s2_2}{s2_3}{s2_0}{s2_1}") {
            return true;
        }
        false       
    }
}