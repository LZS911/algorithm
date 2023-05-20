impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut count: i32 = 0;
        let mut x: i32 = 0;
        let mut used: Vec<bool> = vec![false; k as usize];
        loop {
            x = ((10 % k) * x + 1 % k) % k;
            count += 1;
            if x == 0 {
                return count;
            } else if used[x as usize] {
                break;
            } else {
                used[x as usize] = true
            }
        }
        -1
    }
}
