struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            dp[i][i] = 1;
            for j in i + 1..n {
                if s[i] == s[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
                }
            }
        }
        n as i32 - dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("zzazz");
        assert_eq!(Solution::min_insertions(s), 0);

        let s = String::from("mbadm");
        assert_eq!(Solution::min_insertions(s), 2);

        let s = String::from("leetcode");
        assert_eq!(Solution::min_insertions(s), 5);
    }
}
