struct Solution;

impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let n = satisfaction.len();
        let mut satisfaction = satisfaction;
        satisfaction.sort_unstable();
        let mut dp = vec![vec![None; n + 1]; n];
        Self::solve(&satisfaction, 0, 1, &mut dp)
    }

    fn solve(satisfaction: &[i32], idx: usize, time: i32, dp: &mut [Vec<Option<i32>>]) -> i32 {
        if idx == satisfaction.len() {
            return 0;
        }
        if let Some(val) = dp[idx][time as usize] {
            return val;
        }
        let res = (Self::solve(satisfaction, idx + 1, time + 1, dp) + time * satisfaction[idx])
            .max(Self::solve(satisfaction, idx + 1, time, dp));
        dp[idx][time as usize] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let satisfaction = vec![-1, -8, 0, 5, -9];
        assert_eq!(Solution::max_satisfaction(satisfaction), 14);

        let satisfaction = vec![4, 3, 2];
        assert_eq!(Solution::max_satisfaction(satisfaction), 20);

        let satisfaction = vec![-1, -4, -5];
        assert_eq!(Solution::max_satisfaction(satisfaction), 0);
    }
}
