impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut last: [i32; 128] = [0; 128];
        let mut num = 0;

        for c in jewels.chars() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    last[c as usize] = 1;
                }
                _ => (),
            }
        }

        for c in stones.chars() {
            num += match c {
                'a'..='z' | 'A'..='Z' => {
                    if last[c as usize] > 0 {
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            }
        }
        num
    }
}
