impl Solution {
    pub fn count_time(time: String) -> i32 {
        let mut t = time.chars().collect::<Vec<_>>();

        let hour = match (t[0], t[1]) {
            ('?', '?') => 24,
            ('?', '0'..='3') => 3,
            ('?', '4'..='9') => 2,
            ('0' | '1', '?') => 10,
            ('2', '?') => 4,
            _ => 1,
        };

        let minute = match (t[3], t[4]) {
            ('?', '?') => 60,
            ('?', _) => 6,
            (_, '?') => 10,
            _ => 1,
        };

        hour * minute
    }
}
