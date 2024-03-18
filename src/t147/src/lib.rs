fn get_row(row_index: i32) -> Vec<i32> {
    let row_index = row_index as usize;

    let mut res = vec![0; row_index + 1];

    res[0] = 1;

    for i in 1..=row_index {
        for j in (1..=i).rev() {
            res[j] += res[j - 1];
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(get_row(0), vec![1]);
        assert_eq!(get_row(1), vec![1, 1]);
        assert_eq!(get_row(2), vec![1, 2, 1]);
        assert_eq!(get_row(3), vec![1, 3, 3, 1]);
        assert_eq!(get_row(4), vec![1, 4, 6, 4, 1]);
    }
}
