struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let m = text1.len();
        let n = text2.len();
        let text1 = text1.chars().collect::<Vec<_>>();
        let text2 = text2.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                if text1[i - 1] == text2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let text1 = String::from("abcde");
        let text2 = String::from("ace");
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);

        let text1 = String::from("abc");
        let text2 = String::from("abc");
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);

        let text1 = String::from("abc");
        let text2 = String::from("def");
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 0);
    }
}
