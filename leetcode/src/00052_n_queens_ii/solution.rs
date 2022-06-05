struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut cols = vec![false; n];
        let mut dia = vec![vec![false; n * 2]; 2];
        Solution::backtrack(n, 0, &mut cols, &mut dia)
    }

    fn backtrack(n: usize, row: usize, cols: &mut Vec<bool>, dia: &mut Vec<Vec<bool>>) -> i32 {
        if row == n {
            return 1;
        }
        let mut res = 0;
        for j in 0..n {
            if cols[j] || dia[0][row + j] || dia[1][n - 1 + row - j] {
                continue;
            }
            cols[j] = true;
            dia[0][row + j] = true;
            dia[1][n - 1 + row - j] = true;
            res += Solution::backtrack(n, row + 1, cols, dia);
            cols[j] = false;
            dia[0][row + j] = false;
            dia[1][n - 1 + row - j] = false;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        assert_eq!(Solution::total_n_queens(n), 2);

        let n = 1;
        assert_eq!(Solution::total_n_queens(n), 1);
    }
}
