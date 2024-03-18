fn least_minutes(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];

    dp[1] = 1;

    for i in 2..=n {
        dp[i] = dp[(i + 1) / 2] + 1;
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(least_minutes(1), 1);
        assert_eq!(least_minutes(2), 2);
        assert_eq!(least_minutes(3), 3);
        assert_eq!(least_minutes(4), 3);
        assert_eq!(least_minutes(5), 4);
        assert_eq!(least_minutes(6), 4);
        assert_eq!(least_minutes(7), 4);
        assert_eq!(least_minutes(8), 4);
        assert_eq!(least_minutes(9), 5);
        assert_eq!(least_minutes(10), 5);
    }
}
