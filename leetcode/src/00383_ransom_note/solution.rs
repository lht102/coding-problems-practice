use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut freq = magazine
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut map, ch| {
                *map.entry(ch).or_default() += 1;
                map
            });
        for ch in ransom_note.chars() {
            match freq.get_mut(&ch) {
                Some(cnt) if *cnt == 0 => return false,
                None => return false,
                Some(cnt) => *cnt -= 1,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ransom_note = String::from("a");
        let magazine = String::from("b");
        assert!(!Solution::can_construct(ransom_note, magazine));

        let ransom_note = String::from("aa");
        let magazine = String::from("ab");
        assert!(!Solution::can_construct(ransom_note, magazine));

        let ransom_note = String::from("aa");
        let magazine = String::from("aab");
        assert!(Solution::can_construct(ransom_note, magazine));
    }
}
