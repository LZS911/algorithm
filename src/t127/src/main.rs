impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut ans = 0;
        let mut pos = 0; // 用来判断元素奇偶
        nums.iter()
            .fold(ans, |mut acc, e| {
                if e > &threshold || e & 1 != pos {
                    pos = 0;
                    acc = 0;
                }
                if e <= &threshold && e & 1 == pos {
                    pos ^= 1; // 奇偶转换
                    acc += 1; // 计数
                    ans = ans.max(acc); // 更新 ans
                }
                return acc;
            })
            .max(ans)
    }
}
