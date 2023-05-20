const MONTHS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let parse = |s: String| -> i32 {
            let mut it = s.split('-').map(|v| v.parse::<i32>().unwrap());
            let (month, day) = (it.next().unwrap(), it.next().unwrap());
            MONTHS[..(month - 1) as usize].iter().sum::<i32>() + day
        };

        let a1 = parse(arrive_alice);
        let a2 = parse(leave_alice);
        let b1 = parse(arrive_bob);
        let b2 = parse(leave_bob);
        let min_day = a1.max(b1);
        let max_day = a2.min(b2);

        0.max(max_day - min_day + 1)
    }
}
