struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let k = 2;
        let n = prices.len();
        let mut dp = vec![vec![0; n]; k + 1];
        for i in 1..=k {
            let mut max_p = dp[i - 1][0] - prices[0];
            for (j, &price) in prices.iter().enumerate().skip(1) {
                dp[i][j] = dp[i][j - 1].max(price + max_p);
                max_p = max_p.max(dp[i - 1][j - 1] - price);
            }
        }
        dp[k][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];
        assert_eq!(Solution::max_profit(prices), 6);

        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(prices), 4);

        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0);
    }
}
