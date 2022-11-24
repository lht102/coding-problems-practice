struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n = 9;
        let mut rows = vec![0; n];
        let mut cols = vec![0; n];
        let mut squares = vec![0; n];
        for (i, arr) in board.iter().enumerate() {
            for (j, &item) in arr.iter().enumerate() {
                if item == '.' {
                    continue;
                }
                let val = (item as u8 - b'0') as i32;
                let k = (i / 3) * 3 + j / 3;
                if Solution::is_exist(rows[i], val)
                    || Solution::is_exist(cols[j], val)
                    || Solution::is_exist(squares[k], val)
                {
                    return false;
                }
                rows[i] = Solution::add_val(rows[i], val);
                cols[j] = Solution::add_val(cols[j], val);
                squares[k] = Solution::add_val(squares[k], val);
            }
        }
        true
    }

    fn is_exist(cur: i32, val: i32) -> bool {
        (cur >> val) & 1 > 0
    }

    fn add_val(cur: i32, val: i32) -> i32 {
        cur | (1 << val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(Solution::is_valid_sudoku(board));

        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!Solution::is_valid_sudoku(board));
    }
}
