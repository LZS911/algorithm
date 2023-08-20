impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        nums.iter()
            .skip(1)
            .fold((nums[0] as i64, 0_i64), |dp, &num| {
                ((dp.0).max(dp.1 + num as i64), (dp.1).max(dp.0 - num as i64))
            })
            .0
    }
}
