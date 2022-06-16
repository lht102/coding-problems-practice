use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut res = (0, 0);
        for i in (0..n).rev() {
            for j in i..n {
                if j == i {
                    dp[i][j] = true;
                } else if j == i + 1 {
                    dp[i][j] = s[i] == s[j];
                } else {
                    dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1];
                }
                if dp[i][j] && j - i + 1 > res.1 - res.0 + 1 {
                    res = (i, j);
                }
            }
        }
        String::from_iter(&s[res.0..=res.1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("babad");
        assert_eq!(Solution::longest_palindrome(s), String::from("aba"));

        let s = String::from("cbbd");
        assert_eq!(Solution::longest_palindrome(s), String::from("bb"));
    }
}
