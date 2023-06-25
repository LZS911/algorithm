impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut reward2_sum = reward2.iter().fold(0, |arr, x| arr + x);

        let mut diff = vec![];
        for idx in 0..reward1.len().max(reward2.len()) {
            let (r1, r2) = (
                reward1.get(idx).unwrap_or(&0),
                reward2.get(idx).unwrap_or(&0),
            );
            diff.push(r1 - r2);
        }

        diff.sort_by(|a, b| b.cmp(a));
        for i in 0..k {
            reward2_sum += *diff.get(i as usize).unwrap_or(&0);
        }

        reward2_sum
    }
}
