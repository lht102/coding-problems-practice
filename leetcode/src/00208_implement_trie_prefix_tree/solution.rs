use std::collections::HashMap;

struct Trie {
    end: bool,
    next: HashMap<char, Trie>,
}

impl Trie {
    fn new() -> Self {
        Self {
            end: false,
            next: HashMap::with_capacity(26),
        }
    }

    fn insert(&mut self, word: String) {
        let mut cur = self;
        for ch in word.chars() {
            cur = cur.next.entry(ch).or_insert_with(Trie::new);
        }
        cur.end = true;
    }

    fn search(&self, word: String) -> bool {
        self.dfs(&word).map_or(false, |trie| trie.end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.dfs(&prefix).is_some()
    }

    fn dfs(&self, word: &str) -> Option<&Trie> {
        let mut cur = self;
        for ch in word.chars() {
            match cur.next.get(&ch) {
                Some(node) => cur = node,
                None => return None,
            }
        }
        Some(cur)
    }
}
