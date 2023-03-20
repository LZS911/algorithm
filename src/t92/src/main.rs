use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn get_folder_names(&self, names: Vec<String>) -> Vec<String> {
        let mut d = std::collections::HashMap::new();
        names
            .into_iter()
            .map(|name| {
                let mut s = name.clone();
                while d.contains_key(&s) {
                    s = format!("{}({})", name, d[&name]);
                    *d.get_mut(&name).unwrap() += 1;
                }
                d.insert(s.clone(), 1);
                s
            })
            .collect()
    }
}

fn main() {
    let s = Solution {};
    s.get_folder_names(0.625);
}
