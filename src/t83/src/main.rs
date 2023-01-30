struct Solution {}

impl Solution {
    pub fn count_even(&self, num: i32) -> i32 {
        if num == 1 {
            return 0;
        }
        let x = num.to_string();
        let mut sum = 0;
        for c in x.chars() {
            let n = c as u8 - '0' as u8;
            if n & 1 == 1 {
                sum ^= 1
            }
        }
        if sum == 1 {
            (num - 1) / 2
        } else {
            num / 2
        }
    }
}

fn main() {
    let s = Solution {};
    println!("result value is {}", s.count_even(4));
}
