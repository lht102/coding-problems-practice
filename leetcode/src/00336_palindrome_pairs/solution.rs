use std::collections::HashMap;

struct Solution;

#[derive(Default)]
struct Trie {
    index: Option<usize>,
    palindrome_indices: Vec<usize>,
    next: HashMap<char, Trie>,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, s: &[char], i: usize) {
        let mut cur = self;
        for (j, &ch) in s.iter().enumerate().rev() {
            if is_palindrome(s, 0, j) {
                cur.palindrome_indices.push(i);
            }
            cur = cur.next.entry(ch).or_default();
        }
        cur.index = Some(i);
        cur.palindrome_indices.push(i);
    }

    fn get_palindrome_pairs(&self, s: &[char], i: usize) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut cur = self;
        for (k, &ch) in s.iter().enumerate() {
            if let Some(j) = cur.index {
                if i != j && is_palindrome(s, k, s.len() - 1) {
                    res.push(vec![i as i32, j as i32]);
                }
            }
            match cur.next.get(&ch) {
                Some(node) => cur = node,
                None => return res,
            }
        }
        for &j in &cur.palindrome_indices {
            if i != j {
                res.push(vec![i as i32, j as i32]);
            }
        }
        res
    }
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let arr = words
            .into_iter()
            .map(|w| w.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut root = Trie::new();
        for (i, w) in arr.iter().enumerate() {
            root.insert(w, i);
        }
        let mut res = Vec::new();
        for (i, w) in arr.iter().enumerate() {
            res.append(&mut root.get_palindrome_pairs(w, i));
        }
        res
    }
}

fn is_palindrome(s: &[char], i: usize, j: usize) -> bool {
    let (mut i, mut j) = (i, j);
    while i < j && s[i] == s[j] {
        i += 1;
        j = j.saturating_sub(1);
    }
    i >= j
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            String::from("abcd"),
            String::from("dcba"),
            String::from("lls"),
            String::from("s"),
            String::from("sssll"),
        ];
        assert_eq!(
            Solution::palindrome_pairs(words),
            vec![vec![0, 1], vec![1, 0], vec![2, 4], vec![3, 2]],
        );

        let words = vec![
            String::from("bat"),
            String::from("tab"),
            String::from("cat"),
        ];
        assert_eq!(
            Solution::palindrome_pairs(words),
            vec![vec![0, 1], vec![1, 0]]
        );

        let words = vec![String::from("a"), String::from("")];
        assert_eq!(
            Solution::palindrome_pairs(words),
            vec![vec![0, 1], vec![1, 0]]
        );
    }
}
