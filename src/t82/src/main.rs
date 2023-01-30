struct Solution {}

impl Solution {
    pub fn max_value(&self, n: i32, index: i32, max_sum: i32) -> i32 {
        let (n, index, max_sum) = (n as usize, index as usize, max_sum as usize);
        let l = index.min(n - index - 1);
        let r = n - 1 - l;
        let s = max_sum - n;
        let s1 = l * l;
        let s2 = r * (r - 1) / 2 + (l + 1) * (2 * r - l) / 2;

        if s <= s1 {
            return (s as f64).sqrt() as i32 + 1;
        } else if s >= s2 {
            return (r + (s - s2) / n + 1) as i32;
        } else {
            let delta_s = (s - s1) as f64;
            let t = (4 * l + 1) as f64 / 2f64;
            let delta_x = ((2f64 * delta_s + t * t).sqrt() - t) as i32;
            delta_x + 1 + l as i32
        }
    }
}

fn main() {
    let s = Solution {};
    println!("result value is {}", s.max_value(4, 2, 6));
}
