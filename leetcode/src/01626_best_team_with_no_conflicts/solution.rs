struct Solution;

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut arr = scores
            .iter()
            .copied()
            .zip(ages.iter().copied())
            .collect::<Vec<_>>();
        arr.sort_unstable_by_key(|k| (k.1, k.0));
        let n = scores.len();
        let mut dp = vec![0; n];
        let mut res = 0;
        for (i, &(score, _)) in arr.iter().enumerate() {
            dp[i] = score;
            for (j, &(prev_score, _)) in arr.iter().take(i).enumerate() {
                if prev_score <= score {
                    dp[i] = dp[i].max(dp[j] + score);
                }
            }
            res = res.max(dp[i]);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let scores = vec![1, 3, 5, 10, 15];
        let ages = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::best_team_score(scores, ages), 34);

        let scores = vec![4, 5, 6, 5];
        let ages = vec![2, 1, 2, 1];
        assert_eq!(Solution::best_team_score(scores, ages), 16);

        let scores = vec![1, 2, 3, 5];
        let ages = vec![8, 9, 10, 1];
        assert_eq!(Solution::best_team_score(scores, ages), 6);
    }
}
