impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut dp = vec![vec![0; n]; n];
        for i in 2..n {
            for j in (0..=i - 2).rev() {
                dp[j][i] = i32::MAX;
                for k in j + 1..i {
                    dp[j][i] =
                        dp[j][i].min(dp[j][k] + dp[k][i] + values[i] * values[j] * values[k]);
                }
            }
        }
        dp[0][n - 1]
    }
}
