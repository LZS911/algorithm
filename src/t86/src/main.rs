use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn evaluate(&self, s: String, knowledge: Vec<Vec<String>>) -> String {
        let (cache, _, mut ret) = (
            knowledge
                .iter()
                .map(|v| (v[0].as_str(), v[1].as_str()))
                .collect::<HashMap<_, _>>(),
            s.chars().collect::<Vec<_>>(),
            vec![],
        );

        let mut l = 0;
        for (r, &ch) in s.as_bytes().iter().enumerate() {
            match ch {
                b'(' => {
                    ret.push(&s[l..r]);
                    l = r;
                }
                b')' => {
                    match cache.get(&s[l + 1..r]) {
                        Some(&val) => ret.push(val),
                        None => ret.push("?"),
                    };
                    l = r + 1;
                }
                _ => {}
            }
        }
        ret.push(if l < s.len() { &s[l..] } else { "" });
        ret.join("")
    }
}

fn main() {
    let s = Solution {};
    let vec1 = vec![
        vec![String::from("name"), String::from("bob")],
        vec![String::from("age"), String::from("two")],
    ];
    let vec2 = vec![vec![String::from("a"), String::from("b")]];
    println!(
        "result value is {}",
        s.evaluate(String::from("(name)is(age)yearsold"), vec1)
    );
    println!(
        "result value is {}",
        s.evaluate(String::from("hi(name)"), vec2)
    );
}
