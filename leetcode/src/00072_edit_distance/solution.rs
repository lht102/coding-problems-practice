struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let m = word1.len();
        let n = word2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for (i, arr) in dp.iter_mut().enumerate().take(m + 1) {
            arr[0] = i;
        }
        for j in 0..=n {
            dp[0][j] = j;
        }
        for i in 1..=m {
            for j in 1..=n {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = (dp[i - 1][j]).min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                }
            }
        }
        dp[m][n] as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let word1 = String::from("horse");
        let word2 = String::from("ros");
        assert_eq!(Solution::min_distance(word1, word2), 3);

        let word1 = String::from("intention");
        let word2 = String::from("execution");
        assert_eq!(Solution::min_distance(word1, word2), 5);
    }
}
