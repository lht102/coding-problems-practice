use std::collections::HashMap;

struct Solution;

#[derive(Default)]
struct Trie {
    is_end: bool,
    next: HashMap<char, Trie>,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, s: &str) {
        let mut cur = self;
        for ch in s.chars() {
            cur = cur.next.entry(ch).or_default();
        }
        cur.is_end = true;
    }

    fn find(&self, word: &str, concatenated_count: usize) -> bool {
        if word.is_empty() {
            return concatenated_count > 1;
        }
        let mut cur = self;
        for (i, ch) in word.char_indices() {
            match cur.next.get(&ch) {
                Some(node) => cur = node,
                None => return false,
            }
            if cur.is_end && self.find(&word[i + 1..], concatenated_count + 1) {
                return true;
            }
        }
        false
    }
}

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut root = Trie::new();
        for word in &words {
            root.insert(word);
        }
        words.into_iter().filter(|w| root.find(w, 0)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            String::from("cat"),
            String::from("cats"),
            String::from("catsdogcats"),
            String::from("dog"),
            String::from("dogcatsdog"),
            String::from("hippopotamuses"),
            String::from("rat"),
            String::from("ratcatdogcat"),
        ];
        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(words),
            vec![
                String::from("catsdogcats"),
                String::from("dogcatsdog"),
                String::from("ratcatdogcat"),
            ]
        );

        let words = vec![
            String::from("cat"),
            String::from("dog"),
            String::from("catdog"),
        ];
        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(words),
            vec![String::from("catdog"),]
        );
    }
}
