struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut res = 0;
        let mut dp = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            for j in i..n {
                if j == i {
                    dp[i][j] = 1;
                } else if j == i + 1 {
                    dp[i][j] = if s[i] == s[j] { 1 } else { 0 };
                } else {
                    dp[i][j] = if s[i] == s[j] && dp[i + 1][j - 1] == 1 {
                        1
                    } else {
                        0
                    };
                }
                res += dp[i][j];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("abc");
        assert_eq!(Solution::count_substrings(s), 3);

        let s = String::from("aaa");
        assert_eq!(Solution::count_substrings(s), 6);
    }
}
