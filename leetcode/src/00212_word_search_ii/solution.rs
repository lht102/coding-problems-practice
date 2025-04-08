use std::collections::HashMap;

struct Solution;

#[derive(Default)]
struct Trie {
    word: Option<String>,
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
        cur.word = Some(s.to_owned());
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = Trie::new();
        for w in words.iter() {
            root.insert(w);
        }
        let mut board = board;
        let mut res = Vec::new();
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                Solution::dfs(&mut board, &mut root, i, j, &mut res);
            }
        }
        res
    }

    fn dfs(board: &mut [Vec<char>], root: &mut Trie, ci: usize, cj: usize, res: &mut Vec<String>) {
        if board[ci][cj] == '1' {
            return;
        }
        if let Some(r) = root.next.get_mut(&board[ci][cj]) {
            let tmp = board[ci][cj];
            board[ci][cj] = '1';
            if let Some(w) = r.word.take() {
                res.push(w);
            }
            if ci > 0 {
                Solution::dfs(board, r, ci - 1, cj, res);
            }
            if ci + 1 < board.len() {
                Solution::dfs(board, r, ci + 1, cj, res);
            }
            if cj > 0 {
                Solution::dfs(board, r, ci, cj - 1, res);
            }
            if cj + 1 < board[ci].len() {
                Solution::dfs(board, r, ci, cj + 1, res);
            }
            board[ci][cj] = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec![
            String::from("oath"),
            String::from("pea"),
            String::from("eat"),
            String::from("rain"),
        ];
        assert_eq!(
            Solution::find_words(board, words),
            vec![String::from("oath"), String::from("eat")]
        );

        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words = vec![String::from("abcb")];
        assert_eq!(Solution::find_words(board, words), Vec::<String>::new());

        let board = vec![vec!['a', 'a']];
        let words = vec![String::from("aaa")];
        assert_eq!(Solution::find_words(board, words), Vec::<String>::new());

        let board = vec![
            vec!['o', 'a', 'b', 'n'],
            vec!['o', 't', 'a', 'e'],
            vec!['a', 'h', 'k', 'r'],
            vec!['a', 'f', 'l', 'v'],
        ];
        let words = vec![String::from("oa"), String::from("oaa")];
        assert_eq!(
            Solution::find_words(board, words),
            vec![String::from("oa"), String::from("oaa")]
        );
    }
}
