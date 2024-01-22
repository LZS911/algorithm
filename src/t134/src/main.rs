impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let n = card_points.len();
        let k = k as usize;
        let mut sum: i32 = card_points.iter().take(k).sum();
        let mut res = sum;
        for i in 1..=k {
            sum += card_points[n - i] - card_points[k - i];
            res = res.max(sum);
        }
        res
    }
}

fn main() {}
