impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }
        let mut hash = [0; 26];
        magazine
            .chars()
            .for_each(|ch| hash[(ch as u8 - 'a' as u8) as usize] += 1);
        ransom_note
            .chars()
            .for_each(|ch| hash[(ch as u8 - 'a' as u8) as usize] -= 1);
        !hash.iter().any(|&x| x < 0)
    }
}
