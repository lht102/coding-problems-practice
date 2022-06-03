struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let t = 2;
        let n = prices.len();
        let mut dp = vec![vec![0; n]; t + 1];
        for k in 1..=t {
            let mut min_p = prices[0];
            for (i, &price) in prices.iter().enumerate().take(n).skip(1) {
                min_p = min_p.min(price - dp[k - 1][i - 1]);
                dp[k][i] = dp[k][i - 1].max(price - min_p);
            }
        }
        dp[t][n - 1]
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
