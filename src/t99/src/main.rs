impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let (mut c, mut r, mut o, mut a) = (0, 0, 0, 0);
        let mut res = 0;
        for frog in croak_of_frogs.chars() {
            match frog {
                'c' => c += 1,
                'r' => {
                    if c == 0 {
                        return -1;
                    }
                    c -= 1;
                    r += 1;
                }
                'o' => {
                    if r == 0 {
                        return -1;
                    }
                    r -= 1;
                    o += 1;
                }
                'a' => {
                    if o == 0 {
                        return -1;
                    }
                    o -= 1;
                    a += 1;
                }
                _k => {
                    if a == 0 {
                        return -1;
                    }
                    res = res.max(c + r + o + a);
                    a -= 1;
                }
            }
        }
        if c + r + o + a == 0 {
            res
        } else {
            -1
        }
    }
}
