struct Solution;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![None; n]; m];
        (0..m)
            .map(|i| Self::solve(&grid, i, 0, &mut dp))
            .max()
            .unwrap()
    }

    fn solve(grid: &[Vec<i32>], i: usize, j: usize, dp: &mut [Vec<Option<i32>>]) -> i32 {
        if let Some(val) = dp[i][j] {
            return val;
        }
        let dirs: &'static [(i32, i32)] = &[(-1, 1), (0, 1), (1, 1)];
        let mut res = 0;
        for (x, y) in dirs {
            let ni = i as i32 + x;
            let nj = j as i32 + y;
            if ni < 0 || ni as usize >= grid.len() || nj < 0 || nj as usize >= grid[i].len() {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if grid[ni][nj] <= grid[i][j] {
                continue;
            }
            res = res.max(1 + Self::solve(grid, ni, nj, dp));
        }
        dp[i][j] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![
            vec![2, 4, 3, 5],
            vec![5, 4, 9, 3],
            vec![3, 4, 2, 11],
            vec![10, 9, 13, 15],
        ];
        assert_eq!(Solution::max_moves(grid), 3);

        let grid = vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]];
        assert_eq!(Solution::max_moves(grid), 0);
    }
}
