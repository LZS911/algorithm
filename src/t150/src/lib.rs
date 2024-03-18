fn is_subsequence(s: String, t: String) -> bool {
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < s.len() && j < t.len() {
        if s.as_bytes()[i] == t.as_bytes()[j] {
            i += 1;
        }

        j += 1
    }

    i == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert!(is_subsequence(
            "abcd".to_string(),
            "zipaebecede".to_string()
        ));

        assert!(!is_subsequence(
            "abcd".to_string(),
            "zipaebecee".to_string()
        ));
    }
}
