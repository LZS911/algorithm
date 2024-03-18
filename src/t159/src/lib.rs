fn reverse_bits(num: i32) -> i32 {
    let str = std::format!("{:b}", num);

    // println!("{}", str);

    let dp: Vec<i32> = Vec::new();

    let mut cnt = 0;

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(reverse_bits(1775), 8);
    }
}
