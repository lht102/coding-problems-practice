use std::collections::HashMap;

struct Solution;

struct Trie {
    index: usize,
    next: HashMap<char, Trie>,
}

impl Trie {
    fn new() -> Self {
        Self {
            index: 0,
            next: HashMap::with_capacity(27),
        }
    }

    fn insert(&mut self, word: &str, index: usize) {
        let mut cur = self;
        for ch in word.chars() {
            cur = cur.next.entry(ch).or_insert(Trie::new());
            cur.index = index;
        }
    }

    fn starts_with(&self, word: &str) -> i32 {
        let mut cur = self;
        for ch in word.chars() {
            match cur.next.get(&ch) {
                Some(node) => cur = node,
                None => return -1,
            }
        }
        cur.index as i32
    }
}

struct WordFilter {
    root: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut root = Trie::new();
        for (i, w) in words.iter().enumerate() {
            for j in 0..w.len() {
                root.insert(
                    &w[j..]
                        .chars()
                        .chain(std::iter::once('#'))
                        .chain(w.chars())
                        .collect::<String>(),
                    i,
                );
            }
        }

        Self { root }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        self.root.starts_with(
            &suffix
                .chars()
                .chain(std::iter::once('#'))
                .chain(prefix.chars())
                .collect::<String>(),
        )
    }
}
