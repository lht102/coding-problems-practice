use std::cmp;

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        if n < 2 {
            return cost[0];
        }
        let mut dp = vec![0; n + 1];
        dp[1] = cost[0];
        dp[2] = cost[1];
        for i in 2..=n {
            dp[i] = cmp::min(dp[i - 1], dp[i - 2]) + cost[i - 1];
        }
        cmp::min(dp[n - 1], dp[n])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cost = vec![10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);

        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
    }
}
