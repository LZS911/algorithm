fn count_bits(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut dp = vec![0; n + 1];

    for i in 0..=n {
        if i % 2 == 0 {
            dp[i] = dp[i / 2];
        } else {
            dp[i] = dp[i - 1] + 1;
        }
    }

    dp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(count_bits(0), vec![0]);
        assert_eq!(count_bits(1), vec![0, 1]);
        assert_eq!(count_bits(2), vec![0, 1, 1]);
        assert_eq!(count_bits(3), vec![0, 1, 1, 2]);
        assert_eq!(count_bits(4), vec![0, 1, 1, 2, 1]);
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
        assert_eq!(count_bits(6), vec![0, 1, 1, 2, 1, 2, 2]);
        assert_eq!(count_bits(7), vec![0, 1, 1, 2, 1, 2, 2, 3]);
        assert_eq!(count_bits(8), vec![0, 1, 1, 2, 1, 2, 2, 3, 1]);
    }
}
