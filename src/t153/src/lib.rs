fn divisor_game(n: i32) -> bool {
    let n = n as usize;
    let mut dp = vec![false; n + 2];

    dp[1] = false;
    dp[2] = true;

    if n < 3 {
        return dp[n];
    }

    for i in 3..=n {
        for x in 1..i {
            if i % x == 0 && !dp[i - x] {
                dp[i] = true;
                break;
            }
        }
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert!(!divisor_game(1));
        assert!(divisor_game(2));
        assert!(!divisor_game(3));
        assert!(divisor_game(4));
        assert!(divisor_game(1000));
    }
}
