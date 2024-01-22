impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut map = vec![vec![]; n];
        for line in &connections {
            let (from, to) = (line[0] as usize, line[1] as usize);
            map[from].push((to, 1));
            map[to].push((from, 0));
        }
        let mut ans = 0;
        fn dfs(from: usize, to: usize, map: &Vec<Vec<(usize, i32)>>, ans: &mut i32) {
            for kvp in &map[from] {
                if kvp.0 != to {
                    *ans += kvp.1;
                    dfs(kvp.0, from, map, ans)
                }
            }
        }
        dfs(0, n, &map, &mut ans);
        ans
    }
}
