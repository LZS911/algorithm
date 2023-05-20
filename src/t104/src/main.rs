impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut mp = std::collections::HashMap::new();

        for i in 0..n {
            let mut s = String::new();

            for j in 0..m {
                s.push_str(&(matrix[i][j] ^ matrix[i][0]).to_string());
            }

            *mp.entry(s).or_insert(0) += 1;
        }

        mp.into_values().max().unwrap()
    }
}
