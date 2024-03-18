fn climb_stairs(n: i32) -> i32 {
    if n < 3 {
        return n;
    }
    let mut a = 1; // x - 2
    let mut b = 2; // x - 1
    let mut res = a + b;

    for _i in 3..n + 1 {
        res = a + b;
        a = b;
        b = res;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
    }
}
