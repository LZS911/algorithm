struct Solution {}

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        potions.sort_unstable();

        let len = potions.len() as i32;

        for s in spells {
            let idx = potions.partition_point(|&x| (x as i64 * s as i64) < success) as i32;
            if idx == len {
                res.push(0);
            } else {
                res.push((len - idx) as i32);
            }
        }

        res
    }
}

fn main() {}
