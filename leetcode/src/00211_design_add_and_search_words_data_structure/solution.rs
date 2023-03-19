use std::collections::BTreeMap;

struct Trie {
    end: bool,
    next: BTreeMap<u8, Trie>,
}

impl Trie {
    fn new() -> Self {
        Self {
            end: false,
            next: BTreeMap::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut cur = self;
        for &ch in word.as_bytes() {
            cur = cur.next.entry(ch).or_insert_with(Trie::new);
        }
        cur.end = true;
    }

    fn search(cur: &Trie, word: &[u8]) -> bool {
        if let Some(&first_ch) = word.first() {
            match first_ch {
                b'.' => {
                    for ch in b'a'..=b'z' {
                        if let Some(node) = cur.next.get(&ch) {
                            if Self::search(node, &word[1..]) {
                                return true;
                            }
                        }
                    }
                    false
                }
                ch => {
                    if let Some(node) = cur.next.get(&ch) {
                        return Self::search(node, &word[1..]);
                    }
                    false
                }
            }
        } else {
            cur.end
        }
    }
}

struct WordDictionary {
    root: Trie,
}

impl WordDictionary {
    fn new() -> Self {
        Self { root: Trie::new() }
    }

    fn add_word(&mut self, word: String) {
        self.root.insert(&word);
    }

    fn search(&self, word: String) -> bool {
        Trie::search(&self.root, word.as_bytes())
    }
}
