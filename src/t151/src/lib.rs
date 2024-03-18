struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => Self::fib(n - 1) + Self::fib(n - 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(Solution::fib(0), 0);
        assert_eq!(Solution::fib(1), 1);
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
        assert_eq!(Solution::fib(5), 5);
        assert_eq!(Solution::fib(6), 8);
    }
}
