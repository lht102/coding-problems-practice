struct Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; 6]; n + 1];
        for j in 1..=5 {
            dp[0][j] = 1;
        }
        for i in 1..=n {
            for j in 1..=5 {
                dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
            }
        }
        dp[n][5]
    }

    fn helper(n: usize, v: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if n == 0 {
            return 1;
        }
        if v == 0 {
            return 0;
        }
        if dp[n][v] != -1 {
            return dp[n][v];
        }
        let skip = Self::helper(n, v - 1, dp);
        let chose = Self::helper(n - 1, v, dp);
        dp[n][v] = chose + skip;
        dp[n][v]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 1;
        assert_eq!(Solution::count_vowel_strings(n), 5);

        let n = 2;
        assert_eq!(Solution::count_vowel_strings(n), 15);

        let n = 33;
        assert_eq!(Solution::count_vowel_strings(n), 66045);
    }
}
