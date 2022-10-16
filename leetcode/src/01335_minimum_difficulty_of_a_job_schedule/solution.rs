struct Solution;

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        if job_difficulty.len() < d {
            -1
        } else {
            Solution::solve(
                &job_difficulty,
                d,
                0,
                &mut vec![vec![None; job_difficulty.len()]; d + 1],
            )
        }
    }

    fn solve(job_difficulty: &[i32], d: usize, i: usize, dp: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if d == 1 {
            return *job_difficulty[i..].iter().max().unwrap();
        }
        if let Some(val) = dp[d][i] {
            return val;
        }
        let mut min_difficulty = i32::MAX;
        let mut cur_difficulty = 0;
        for j in i..job_difficulty.len() - d + 1 {
            cur_difficulty = cur_difficulty.max(job_difficulty[j]);
            min_difficulty = min_difficulty
                .min(cur_difficulty + Solution::solve(job_difficulty, d - 1, j + 1, dp));
        }
        dp[d][i] = Some(min_difficulty);
        min_difficulty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let job_difficulty = vec![6, 5, 4, 3, 2, 1];
        let d = 2;
        assert_eq!(Solution::min_difficulty(job_difficulty, d), 7);

        let job_difficulty = vec![9, 9, 9];
        let d = 4;
        assert_eq!(Solution::min_difficulty(job_difficulty, d), -1);

        let job_difficulty = vec![1, 1, 1];
        let d = 3;
        assert_eq!(Solution::min_difficulty(job_difficulty, d), 3);
    }
}
