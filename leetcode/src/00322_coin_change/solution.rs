struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;
        for &coin in &coins {
            for j in coin..=amount {
                dp[j as usize] = dp[j as usize].min(dp[(j - coin) as usize] + 1);
            }
        }
        if dp[amount as usize] == amount + 1 {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        assert_eq!(Solution::coin_change(coins, amount), 3);

        let coins = vec![2];
        let amount = 3;
        assert_eq!(Solution::coin_change(coins, amount), -1);

        let coins = vec![1];
        let amount = 0;
        assert_eq!(Solution::coin_change(coins, amount), 0);
    }
}
