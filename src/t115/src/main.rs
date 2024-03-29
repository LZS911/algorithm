impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut carry = 0;
        let mut res = vec![];
        let (mut i, mut j) = (num1.len() as i32 - 1, num2.len() as i32 - 1);
        let (num1, num2) = (num1.as_bytes(), num2.as_bytes());
        while i >= 0 || j >= 0 {
            let x = if i >= 0 { num1[i as usize] - b'0' } else { 0 }
                + if j >= 0 { num2[j as usize] - b'0' } else { 0 }
                + carry;
            carry = x / 10;
            res.push((x % 10 + b'0') as char);
            i -= 1;
            j -= 1;
        }
        if carry != 0 {
            res.push((carry + b'0') as char);
        }
        res.iter().rev().collect()
    }
}
