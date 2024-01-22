struct Solution {}

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut pos = vec![(0, 0); arr.len() + 1];
        let mut row_cnt = vec![mat[0].len(); mat.len()];
        let mut col_cnt = vec![mat.len(); mat[0].len()];
        mat.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &x)| {
                pos[x as usize] = (i, j);
            });
        });
        for (i, &n) in arr.iter().enumerate() {
            let (r, c) = pos[n as usize];
            row_cnt[r] -= 1;
            col_cnt[c] -= 1;
            if row_cnt[r] == 0 || col_cnt[c] == 0 {
                return i as i32;
            }
        }
        0
    }
}

fn main() {}
