impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1_usize..nums.len() {
            if nums[i - 1] == nums[i] {
                nums[i - 1] *= 2;
                nums[i] = 0;
            }
        }
        for i in 0_usize..nums.len() {
            for j in 0_usize..nums.len() - 1_usize {
                if (nums[j] == 0 && nums[j + 1_usize] != 0) {
                    nums.swap(j, j + 1_usize);
                }
            }
        }
        nums
    }
}
