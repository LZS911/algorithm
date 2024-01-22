impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        (1 + n..)
            .find(|&x| {
                let mut x = x;
                let mut cnt = vec![0; 10];
                while x > 0 {
                    cnt[(x % 10) as usize] += 1;
                    x /= 10;
                }
                cnt.into_iter()
                    .enumerate()
                    .filter(|&(_, c)| c != 0)
                    .all(|(i, c)| i == c)
            })
            .unwrap()
    }
}
