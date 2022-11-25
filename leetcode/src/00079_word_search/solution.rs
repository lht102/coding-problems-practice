struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let word = word.chars().collect::<Vec<_>>();
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if Solution::dfs(&mut board, &word, 0, i as _, j as _) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, word: &[char], k: usize, ci: i32, cj: i32) -> bool {
        if k == word.len() {
            return true;
        }
        if ci < 0
            || ci >= board.len() as _
            || cj < 0
            || cj >= board[ci as usize].len() as _
            || word[k] != board[ci as usize][cj as usize]
        {
            return false;
        }
        let tmp = board[ci as usize][cj as usize];
        board[ci as usize][cj as usize] = '*';
        let found = Solution::dfs(board, word, k + 1, ci - 1, cj)
            || Solution::dfs(board, word, k + 1, ci + 1, cj)
            || Solution::dfs(board, word, k + 1, ci, cj - 1)
            || Solution::dfs(board, word, k + 1, ci, cj + 1);
        board[ci as usize][cj as usize] = tmp;
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCCED");
        assert!(Solution::exist(board, word));

        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("SEE");
        assert!(Solution::exist(board, word));

        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCB");
        assert!(!Solution::exist(board, word));

        let board = vec![vec!['A']];
        let word = String::from("A");
        assert!(Solution::exist(board, word));
    }
}
