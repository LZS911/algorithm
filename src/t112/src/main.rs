impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut ans = 1e7 as i32;
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == target {
                    return target;
                }
                if (sum - target).abs() < (ans - target).abs() {
                    ans = sum;
                }
                if sum > target {
                    k -= 1;
                    while k > j && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                } else if sum < target {
                    j += 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                }
            }
        }
        ans
    }
}
