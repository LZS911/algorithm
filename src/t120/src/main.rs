impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut minDp, mut maxDp) = (vec![0; n + 1], vec![0; n + 1]);
        let mut res = 0;
        for i in 0..n {
            let num = nums[i];
            maxDp[i + 1] = (maxDp[i] + num).max(num);
            minDp[i + 1] = (minDp[i] + num).min(num);
            res = res.max(maxDp[i + 1].max(minDp[i + 1].abs()));
        }
        res
    }
}
