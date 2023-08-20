impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut block = false;
        let mut ans = vec![];
        let mut pre = String::new();
        let mut iter = source.into_iter();
        let mut now = iter.next();
        while let Some(line) = now {
            if block {
                let index = line.find("*/");
                if let Some(i) = index.map(|it| it + 2) {
                    block = false;
                    pre.push_str(&line[i..]);
                    if !pre.is_empty() {
                        now = Some(pre.clone());
                        continue;
                    }
                    pre = String::new();
                }
                now = iter.next();
                continue;
            }
            let li = line.find("//").unwrap_or(line.len() + 1);
            let bi = line.find("/*").unwrap_or(line.len() + 1);
            if li < bi {
                ans.push(String::from(&line[..li]));
                now = iter.next();
            } else if bi < li {
                pre = (&line[..bi]).to_string();
                block = true;
                now = Some((&line[(bi + 2)..]).to_string());
            } else {
                ans.push(line.clone());
                now = iter.next();
            }
        }
        ans.into_iter().filter(|it| !it.is_empty()).collect()
    }
}
