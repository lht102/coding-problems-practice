struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let m = 3.max(n as usize);
        let mut dp = vec![0; m + 1];
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 1;
        for i in 3..=m {
            dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        assert_eq!(Solution::tribonacci(n), 4);

        let n = 25;
        assert_eq!(Solution::tribonacci(n), 1389537);
    }
}
