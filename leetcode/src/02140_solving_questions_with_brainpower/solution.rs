struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![0; questions.len() + 1];
        Self::solve(&questions, 0, &mut dp);
        dp[0]
    }

    fn solve(questions: &Vec<Vec<i32>>, i: usize, dp: &mut [i64]) -> i64 {
        if i >= questions.len() {
            return 0;
        }
        if dp[i] != 0 {
            return dp[i];
        }
        let points_without_skip =
            Self::solve(questions, i + questions[i][1] as usize + 1, dp) + questions[i][0] as i64;
        let points_with_skip = Self::solve(questions, i + 1, dp);
        dp[i] = points_without_skip.max(points_with_skip);
        dp[i]
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
