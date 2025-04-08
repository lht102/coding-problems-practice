use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words = s.split_whitespace().collect::<Vec<_>>();
        if pattern.len() != words.len() {
            return false;
        }
        let mut h1 = HashMap::<char, &str>::new();
        let mut h2 = HashMap::<&str, char>::new();
        for (i, ch) in pattern.char_indices() {
            let word = words[i];
            if let Some(&matched_word) = h1.get(&ch) {
                if matched_word != word {
                    return false;
                }
                if *h2.get(matched_word).unwrap() != ch {
                    return false;
                }
            } else if h2.contains_key(word) {
                return false;
            } else {
                h1.insert(ch, word);
                h2.insert(word, ch);
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
        let pattern = String::from("abba");
        let s = String::from("dog cat cat dog");
        assert!(Solution::word_pattern(pattern, s));

        let pattern = String::from("abba");
        let s = String::from("dog cat cat fish");
        assert!(!Solution::word_pattern(pattern, s));

        let pattern = String::from("aaaa");
        let s = String::from("dog cat cat dog");
        assert!(!Solution::word_pattern(pattern, s));

        let pattern = String::from("abba");
        let s = String::from("dog dog dog dog");
        assert!(!Solution::word_pattern(pattern, s));

        let pattern = String::from("abc");
        let s = String::from("dog cat dog");
        assert!(!Solution::word_pattern(pattern, s));
    }
}
