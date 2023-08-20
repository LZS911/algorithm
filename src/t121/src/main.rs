impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let sum = mat.len() - 1;

        mat.iter()
            .enumerate()
            .map(|(n, vec)| {
                let (x, y) = (n, sum - n);

                if x == y {
                    vec[n]
                } else {
                    vec[x] + vec[y]
                }
            })
            .sum()
    }
}
