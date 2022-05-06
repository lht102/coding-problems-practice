use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
        let mut seen = HashSet::<i32>::new();
        for w in &start_words {
            let mut mask = 0;
            for ch in w.chars() {
                mask ^= 1 << (ch as u8 - b'a')
            }
            seen.insert(mask);
        }
        let mut res = 0;
        for w in &target_words {
            let mut mask = 0;
            for ch in w.chars() {
                mask ^= 1 << (ch as u8 - b'a')
            }
            for ch in w.chars() {
                if seen.contains(&(mask ^ (1 << (ch as u8 - b'a')))) {
                    res += 1;
                    break;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let start_words = vec![
            String::from("ant"),
            String::from("act"),
            String::from("tack"),
        ];
        let target_words = vec![
            String::from("tack"),
            String::from("act"),
            String::from("acti"),
        ];
        assert_eq!(Solution::word_count(start_words, target_words), 2);

        let start_words = vec![String::from("ab"), String::from("a")];
        let target_words = vec![String::from("abc"), String::from("abcd")];
        assert_eq!(Solution::word_count(start_words, target_words), 1);
    }
}
