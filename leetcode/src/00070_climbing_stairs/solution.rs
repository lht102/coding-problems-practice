struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 2;
        assert_eq!(Solution::climb_stairs(n), 2);

        let n = 3;
        assert_eq!(Solution::climb_stairs(n), 3);
    }
}
