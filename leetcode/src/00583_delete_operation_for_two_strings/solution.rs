struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        for i in 0..word1.len() {
            dp[i + 1][0] = i as i32 + 1;
        }
        for j in 0..word2.len() {
            dp[0][j + 1] = j as i32 + 1;
        }
        for (i, ch1) in word1.char_indices() {
            for (j, ch2) in word2.char_indices() {
                dp[i + 1][j + 1] = (dp[i][j + 1] + 1)
                    .min(dp[i + 1][j] + 1)
                    .min(dp[i][j] + if ch1 == ch2 { 0 } else { 2 });
            }
        }
        dp[word1.len()][word2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let word1 = String::from("sea");
        let word2 = String::from("eat");
        assert_eq!(Solution::min_distance(word1, word2), 2);

        let word1 = String::from("leetcode");
        let word2 = String::from("etco");
        assert_eq!(Solution::min_distance(word1, word2), 4);
    }
}
