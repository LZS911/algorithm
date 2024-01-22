impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let mut bits_len = matrix[0].len();
        let tab: Vec<_> = matrix
            .into_iter()
            .map(|a| {
                a.into_iter()
                    .enumerate()
                    .fold(0, |x, (i, b)| (x | (b << i) as u32))
            })
            .collect();
        let mut ans = 0;
        for x in 0u32..1 << bits_len {
            if x.count_ones() != num_select as u32 {
                continue;
            }
            let cnt = tab.iter().fold(0, |cnt, &a| cnt + ((a & x) == a) as i32);
            ans = ans.max(cnt);
        }
        ans
    }
}
