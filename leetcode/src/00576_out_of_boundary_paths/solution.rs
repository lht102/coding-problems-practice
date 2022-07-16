struct Solution;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut dp = vec![vec![vec![-1; max_move as usize + 1]; n as usize]; m as usize];
        Solution::solve(m, n, max_move, start_row, start_column, &mut dp)
    }

    fn solve(
        m: i32,
        n: i32,
        remaining_move: i32,
        ci: i32,
        cj: i32,
        dp: &mut [Vec<Vec<i32>>],
    ) -> i32 {
        if ci < 0 || ci >= m || cj < 0 || cj >= n {
            return 1;
        }
        if remaining_move == 0 {
            return 0;
        }
        let (ci_usize, cj_usize, remaining_move_usize) =
            (ci as usize, cj as usize, remaining_move as usize);
        if dp[ci_usize][cj_usize][remaining_move_usize] != -1 {
            return dp[ci_usize][cj_usize][remaining_move_usize];
        }
        let dirs: &'static [i32] = &[0, 1, 0, -1, 0];
        const M: i32 = 1000000007;
        let mut res = 0;
        for i in 0..4 {
            res = (res
                + Solution::solve(m, n, remaining_move - 1, ci + dirs[i], cj + dirs[i + 1], dp)
                    % M)
                % M;
        }
        dp[ci_usize][cj_usize][remaining_move_usize] = res;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let m = 2;
        let n = 2;
        let max_move = 2;
        let start_row = 0;
        let start_column = 0;
        assert_eq!(
            Solution::find_paths(m, n, max_move, start_row, start_column),
            6
        );

        let m = 1;
        let n = 3;
        let max_move = 3;
        let start_row = 0;
        let start_column = 1;
        assert_eq!(
            Solution::find_paths(m, n, max_move, start_row, start_column),
            12
        );
    }
}
