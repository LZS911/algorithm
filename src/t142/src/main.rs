impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut dic = std::collections::BTreeMap::new();
        for c in s.chars() {
            dic.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        let mut ans = Vec::with_capacity(s.len());

        while dic.len() > 1 {
            if let Some((key, val)) = dic.pop_last() {
                if ans.len() > 0 && ans[ans.len() - 1] == key {
                    let prv = (key, val);
                    let current = dic.pop_last().unwrap();
                    ans.push(current.0);
                    dic.insert(prv.0, prv.1);
                    if current.1 > 1 {
                        dic.insert(current.0, current.1 - 1);
                    }
                } else {
                    if val > repeat_limit {
                        for _ in 0..repeat_limit {
                            ans.push(key);
                        }
                        dic.insert(key, val - repeat_limit);
                    } else {
                        for _ in 0..val {
                            ans.push(key);
                        }
                    }
                }
            };
        }
        if !dic.is_empty() {
            let current = dic.pop_last().unwrap();
            if ans.is_empty() || ans[ans.len() - 1] != current.0 {
                for _ in 0..current.1.min(repeat_limit) {
                    ans.push(current.0);
                }
            }
        }
        ans.iter().collect()
    }
}
