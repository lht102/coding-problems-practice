struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut cols = vec![false; n];
        let mut dia = vec![vec![false; n * 2]; 2];
        let mut cur = vec![vec!['.'; n]; n];
        let mut res = Vec::new();
        Solution::backtrack(n, 0, &mut cols, &mut dia, &mut cur, &mut res);
        res
    }

    fn backtrack(
        n: usize,
        i: usize,
        cols: &mut Vec<bool>,
        dia: &mut Vec<Vec<bool>>,
        cur: &mut Vec<Vec<char>>,
        res: &mut Vec<Vec<String>>,
    ) {
        if i == n {
            res.push(
                cur.iter()
                    .map(|chs| chs.iter().collect::<String>())
                    .collect::<Vec<_>>(),
            );
            return;
        }
        for j in 0..n {
            if Solution::is_valid(n, i, j, cols, dia) {
                cols[j] = true;
                dia[0][i + j] = true;
                dia[1][n - j + i - 1] = true;
                cur[i][j] = 'Q';
                Solution::backtrack(n, i + 1, cols, dia, cur, res);
                cur[i][j] = '.';
                cols[j] = false;
                dia[0][i + j] = false;
                dia[1][n - j + i - 1] = false;
            }
        }
    }

    fn is_valid(n: usize, i: usize, j: usize, cols: &[bool], dia: &[Vec<bool>]) -> bool {
        !cols[j] && !dia[0][i + j] && !dia[1][n - j + i - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        assert_eq!(
            Solution::solve_n_queens(n),
            vec![
                vec![
                    String::from(".Q.."),
                    String::from("...Q"),
                    String::from("Q..."),
                    String::from("..Q."),
                ],
                vec![
                    String::from("..Q."),
                    String::from("Q..."),
                    String::from("...Q"),
                    String::from(".Q..")
                ]
            ]
        );

        let n = 1;
        assert_eq!(Solution::solve_n_queens(n), vec![vec![String::from("Q"),],]);
    }
}
