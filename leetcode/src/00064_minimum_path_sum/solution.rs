struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut dp = vec![0; n + 1];
        for j in 0..n {
            dp[j + 1] = dp[j] + grid[0][j];
        }
        dp[0] = i32::MAX;
        for arr in grid.iter().skip(1) {
            for j in 0..n {
                dp[j + 1] = dp[j + 1].min(dp[j]) + arr[j];
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(Solution::min_path_sum(grid), 7);

        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(Solution::min_path_sum(grid), 12);
    }
}
