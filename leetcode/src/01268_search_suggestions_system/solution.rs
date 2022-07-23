use std::collections::HashMap;

struct Solution;

#[derive(Default)]
struct Trie {
    indices: Vec<usize>,
    next: HashMap<char, Trie>,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, s: &str, index: usize) {
        let mut cur = self;
        for ch in s.chars() {
            cur = cur.next.entry(ch).or_insert_with(Trie::new);
            cur.indices.push(index);
        }
    }

    fn starts_with(&self, s: &str) -> Vec<usize> {
        let mut cur = self;
        for ch in s.chars() {
            match cur.next.get(&ch) {
                Some(node) => cur = node,
                None => return Vec::new(),
            }
        }
        cur.indices.iter().take(3).cloned().collect()
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort_unstable();
        let mut root = Trie::new();
        for (i, p) in products.iter().enumerate() {
            root.insert(p, i);
        }
        let mut res = Vec::with_capacity(search_word.len());
        for i in 1..=search_word.len() {
            res.push(
                root.starts_with(&search_word[..i])
                    .iter()
                    .map(|&j| products[j].clone())
                    .collect(),
            );
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let products = vec![
            String::from("mobile"),
            String::from("mouse"),
            String::from("moneypot"),
            String::from("monitor"),
            String::from("mousepad"),
        ];
        let search_word = String::from("mouse");
        assert_eq!(
            Solution::suggested_products(products, search_word),
            vec![
                vec![
                    String::from("mobile"),
                    String::from("moneypot"),
                    String::from("monitor"),
                ],
                vec![
                    String::from("mobile"),
                    String::from("moneypot"),
                    String::from("monitor"),
                ],
                vec![String::from("mouse"), String::from("mousepad"),],
                vec![String::from("mouse"), String::from("mousepad"),],
                vec![String::from("mouse"), String::from("mousepad"),]
            ]
        );
    }
}
