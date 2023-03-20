use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn min_operations_max_profit(
        &self,
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        let (mut max_profit, mut cur_profit, mut wait_customers, mut index, mut ret) =
            (0, -1, 0, 0, -1);
        if boarding_cost * 4 < running_cost {
            return ret;
        }
        while index < customers.len() || wait_customers > 0 {
            if index < customers.len() {
                wait_customers += customers[index];
            }
            index += 1;
            let cnt = 4.min(wait_customers);
            cur_profit += cnt * boarding_cost - running_cost;
            wait_customers -= cnt;
            if cur_profit > max_profit {
                max_profit = cur_profit;
                ret = index as i32;
            }
        }
        ret
    }
}

fn main() {
    let s = Solution {};
}
