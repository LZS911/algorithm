use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn gap(h1: i32, h2: i32, m1: i32, m2: i32) -> i32 {
        (h2 - h1) * 60 - m1 + m2
    }
    fn judge_is_right(nums: &Vec<(i32, i32)>) -> bool {
        let n = nums.len();
        if n < 3 {
            return true;
        }
        for index in 2..n {
            let gap = Solution::gap(
                nums[index - 2].0,
                nums[index].0,
                nums[index - 2].1,
                nums[index].1,
            );
            if gap <= 60 {
                return true;
            }
        }
        false
    }
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut map: HashMap<&String, Vec<(i32, i32)>> = HashMap::new();
        for index in 0..key_name.len() {
            let value = map.entry(&key_name[index]).or_insert(vec![]);
            let tmp: Vec<&str> = key_time[index].split(':').collect();
            (*value).push((
                tmp[0].parse::<i32>().unwrap(),
                tmp[1].parse::<i32>().unwrap(),
            ));
        }
        let mut res: Vec<String> = vec![];
        for (name, mut value) in map {
            value.sort();
            if Solution::judge_is_right(&value) {
                res.push(name.clone());
            }
        }
        res.sort();
        res
    }
}
