impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let n = flips.len();
        let mut ans = 0;
        let mut max = 0;

        for i in 0..n {
            max = max.max(flips[i]);

            if max == i as i32 + 1 {
                ans += 1;
            }
        }

        ans
    }
}
