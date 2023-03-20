use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; grid[0].len() + 1];
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                dp[col + 1] = grid[row][col] + dp[col].max(dp[col + 1]);
            }
        }
        *dp.last().unwrap()
    }
}

fn main() {
    let s = Solution {};
}
