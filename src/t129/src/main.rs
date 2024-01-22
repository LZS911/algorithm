impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut min_pre_sum = 0;
        let mut pre_sum = 0;
        for &x in &nums {
            pre_sum += x; // 当前的前缀和
            ans = ans.max(pre_sum - min_pre_sum); // 减去前缀和的最小值
            min_pre_sum = min_pre_sum.min(pre_sum); // 维护前缀和的最小值
        }
        ans
    }
}
