struct Solution {}

impl Solution {
    pub fn reinitialize_permutation(&self, n: i32) -> i32 {
        if n == 2 {
            return 1;
        }

        let mut step = 1;
        let mut pow = 2;

        while pow != 1 {
            step += 1;
            pow = pow * 2 % (n - 1);
        }

        step
    }
}

fn main() {
    let s = Solution {};
    println!("result value is {}", s.reinitialize_permutation(2));
    println!("result value is {}", s.reinitialize_permutation(4));
}
