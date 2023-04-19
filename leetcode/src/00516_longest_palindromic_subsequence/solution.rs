struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![None; n]; n];
        Self::solve(s.as_bytes(), 0, n - 1, &mut dp)
    }

    fn solve(s: &[u8], i: usize, j: usize, dp: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if i > j {
            return 0;
        }
        if i == j {
            return 1;
        }
        if let Some(val) = dp[i][j] {
            return val;
        }
        let res = if s[i] == s[j] {
            2 + Self::solve(s, i + 1, j.saturating_sub(1), dp)
        } else {
            Self::solve(s, i + 1, j, dp).max(Self::solve(s, i, j.saturating_sub(1), dp))
        };
        dp[i][j] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("bbbab");
        assert_eq!(Solution::longest_palindrome_subseq(s), 4);

        let s = String::from("cbbd");
        assert_eq!(Solution::longest_palindrome_subseq(s), 2);
    }
}
