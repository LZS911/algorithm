fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
    let k = k as usize;
    let n = n as usize;

    let mut dp = vec![vec![0; n - 1]; k + 1];
    dp[0][0] = 1;

    for i in 0..k {
        // k - 1 步时，res = relation[i][n - 2] 的数量
        // k - 2 步时，res = relation[n-2] 的数量

        for item in &relation {
            let source = item[0] as usize;
            let target = item[1] as usize;

            dp[i + 1][target] += dp[i][source];
        }
    }

    dp[k][n - 1]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {}
}
