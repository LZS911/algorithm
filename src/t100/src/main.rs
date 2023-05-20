impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 60];
        let mut ans = 0;

        for mut t in time {
            t %= 60;
            ans += cnt[(60 - t as usize) % 60];
            cnt[t as usize] += 1;
        }

        ans
    }
}
