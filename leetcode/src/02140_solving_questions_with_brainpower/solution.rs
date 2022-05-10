struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![-1; questions.len() + 1];
        Self::dfs(&questions, 0, &mut dp);
        dp[0]
    }

    fn dfs(questions: &Vec<Vec<i32>>, ci: usize, dp: &mut Vec<i64>) -> i64 {
        if ci >= questions.len() {
            return 0;
        }
        if dp[ci] != -1 {
            return dp[ci];
        }
        let solve =
            Self::dfs(questions, ci + questions[ci][1] as usize + 1, dp) + questions[ci][0] as i64;
        let skip = Self::dfs(questions, ci + 1, dp);
        dp[ci] = solve.max(skip);
        dp[ci]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
        assert_eq!(Solution::most_points(questions), 5);

        let questions = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        assert_eq!(Solution::most_points(questions), 7);
    }
}
