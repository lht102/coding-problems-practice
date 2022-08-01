struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1; n];
        for _ in 1..m {
            for j in 1..n {
                dp[j] += dp[j - 1];
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let m = 3;
        let n = 7;
        assert_eq!(Solution::unique_paths(m, n), 28);

        let m = 3;
        let n = 2;
        assert_eq!(Solution::unique_paths(m, n), 3);
    }
}
