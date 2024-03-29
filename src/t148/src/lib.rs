fn max_profit(prices: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut min_price = prices[0];

    for price in prices {
        min_price = min_price.min(price);
        res = res.max(price - min_price);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
