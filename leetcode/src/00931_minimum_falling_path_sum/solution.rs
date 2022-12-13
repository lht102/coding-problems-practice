struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp = matrix;
        let m = dp.len();
        for i in 1..dp.len() {
            let n = dp[i].len();
            if n > 1 {
                dp[i][0] += dp[i - 1][0].min(dp[i - 1][1]);
                dp[i][n - 1] += dp[i - 1][n - 1].min(dp[i - 1][n - 2]);
            } else {
                dp[i][0] += dp[i - 1][0];
                dp[i][n - 1] += dp[i - 1][n - 1];
            }
            for j in 1..n - 1 {
                dp[i][j] += dp[i - 1][j].min(dp[i - 1][j - 1]).min(dp[i - 1][j + 1]);
            }
        }
        *dp[m - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        assert_eq!(Solution::min_falling_path_sum(matrix), 13);

        let matrix = vec![vec![-19, 57], vec![-40, -5]];
        assert_eq!(Solution::min_falling_path_sum(matrix), -59);
    }
}
