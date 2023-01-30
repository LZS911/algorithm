struct Solution {}

impl Solution {
    pub fn digit_count(&self, num: String) -> bool {
        let (mut cnt, ch_arr) = (vec![0; 11], num.as_bytes());
        for &ch in ch_arr {
            cnt[(ch - b'0') as usize] += 1;
        }
        for i in 0..ch_arr.len() {
            if (ch_arr[i] - b'0') as i32 != cnt[i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let s = Solution {};
    println!("result value is {}", s.digit_count(String::from("1210")));
    println!("result value is {}", s.digit_count(String::from("030")));
}
