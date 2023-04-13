impl Solution {
    pub fn mask_pii(s: String) -> String {
        const COUNTRY_CODE: [&str; 4] = ["", "+*-", "+**-", "+＊＊＊-"];
        match s.find('@') {
            Some(idx) => s[0..1].to_lowercase() + "＊＊＊**" + &s[(idx - 1)..].to_lowercase(),
            None => {
                let numbers = s.matches(char::is_numeric).collect::<String>();
                let n = numbers.len();
                COUNTRY_CODE[n - 10].to_string() + "＊＊＊-＊＊＊-" + &numbers[(n - 4)..]
            }
        }
    }
}
