pub struct Solution {}

impl Solution {
    pub fn furthest_distance_from_origin_1(moves: String) -> i32 {
        let (mut l_count, mut r_count, mut _count) = (0, 0, 0);
        for m in moves.chars() {
            match m {
                'L' => l_count += 1,
                'R' => r_count += 1,
                '_' => _count += 1,
                _ => {}
            }
        }
        (l_count as i32 - r_count).abs() + _count
    }
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let [l_count, r_count, _count] = moves.chars().fold([0, 0, 0], |mut acc, m| {
            match m {
                'L' => acc[0] += 1,
                'R' => acc[1] += 1,
                '_' => acc[2] += 1,
                _ => {}
            }
            acc
        });
        (l_count as i32 - r_count).abs() + _count
    }
}
