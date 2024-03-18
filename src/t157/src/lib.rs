fn max_sales(sales: Vec<i32>) -> i32 {
    let len = sales.len();

    // let mut dp = vec![std::i32::MIN; len + 1];

    // dp[0] = sales[0];

    // for i in 1..len {
    //     dp[i] = sales[i].max(dp[i - 1] + sales[i]);
    // }

    // *dp.iter().max().unwrap()

    let mut max = sales[0];
    let mut pre = 0;

    for i in 0..len {
        pre = sales[i].max(sales[i] + pre);
        max = max.max(pre);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(max_sales(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6)
    }
}
