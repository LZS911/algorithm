fn count_bits(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        if i % 2 == 1 {
            dp[i] = dp[i - 1] + 1;
        } else {
            dp[i] = dp[i / 2];
        }
    }

    dp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(count_bits(6), vec![0, 1, 1, 2, 1, 2, 2]);
    }
}
