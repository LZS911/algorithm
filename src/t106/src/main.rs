use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut cnt = HashMap::new();

        for row in grid.iter() {
            *cnt.entry(row).or_insert(0) += 1;
        }

        let mut res = 0;
        let n = grid.len();
        for j in 0..n {
            let mut arr = Vec::new();
            for i in 0..n {
                arr.push(grid[i][j]);
            }
            res += cnt.get(&arr).unwrap_or(&0);
        }

        res
    }
}
