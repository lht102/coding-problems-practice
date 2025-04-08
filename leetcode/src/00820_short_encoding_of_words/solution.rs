use std::collections::HashMap;
use std::collections::VecDeque;

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
}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut root = Trie::new();
        for w in &words {
            root.insert(&w.chars().rev().collect::<String>());
        }
        let mut res = 0;
        let mut level = 0;
        let mut q = VecDeque::new();
        q.push_back(&root);
        while !q.is_empty() {
            for _ in 0..q.len() {
                let u = q.pop_front().unwrap();
                if u.next.is_empty() {
                    res += level + 1;
                } else {
                    for v in u.next.values() {
                        q.push_back(v);
                    }
                }
            }
            level += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            String::from("time"),
            String::from("me"),
            String::from("bell"),
        ];
        assert_eq!(Solution::minimum_length_encoding(words), 10);

        let words = vec![String::from("t")];
        assert_eq!(Solution::minimum_length_encoding(words), 2);
    }
}
