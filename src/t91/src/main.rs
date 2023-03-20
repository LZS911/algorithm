struct Solution {}

impl Solution {
    pub fn print_bin(&self, num: f64) -> String {
        let mut res: String = String::from("0.");
        let mut num = num;
        while num != 0_f64 {
            num *= 2_f64;
            if num >= 1_f64 {
                res.push('1');
                num -= 1_f64;
            } else {
                res.push('0');
            }
            if res.len() > 32 {
                return "ERROR".to_string();
            }
        }
        res
    }
}

fn main() {
    let s = Solution {};
    s.print_bin(0.625);
}
