fn tribonacci(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];

    if n == 0 {
        return 0;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    dp[0] = 0;
    dp[1] = 1;
    dp[2] = 1;

    for i in 3..=n {
        dp[i] = dp[i - 3] + dp[i - 2] + dp[i - 1];
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(tribonacci(0), 0);
        assert_eq!(tribonacci(1), 1);
        assert_eq!(tribonacci(2), 1);
        assert_eq!(tribonacci(3), 2);
        assert_eq!(tribonacci(4), 4);
        assert_eq!(tribonacci(5), 7);
        assert_eq!(tribonacci(25), 1389537);
    }
}
