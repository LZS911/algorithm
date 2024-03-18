fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let len = cost.len();
    // let mut dp = vec![0; len + 1];
    let mut cost_one = 0;
    let mut cost_two = 0;

    for i in 2..=len {
        // dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
        let mut sum = 0;
        let sum = (cost_one + cost[i - 1]).min(cost_two + cost[i - 2]);
        cost_two = cost_one;
        cost_one = sum;
    }
    cost_one
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
