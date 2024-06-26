pub fn num_trees(n: i32) -> i32 {
    let len: usize = n as usize;
    let mut dp = vec![0; len + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..len + 1 {
        for j in 1..i + 1 {
            dp[i] += dp[j - 1] * dp[i - j];
        }
    }
    return dp[len];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(num_trees(3), 5);
        assert_eq!(num_trees(1), 1)
    }
}
