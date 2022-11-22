struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let arr = Solution::square_numbers(n);
        let mut dp = vec![n + 1; n + 1];
        dp[0] = 0;
        for &val in &arr {
            for i in val..=n {
                dp[i] = dp[i].min(dp[i - val] + 1);
            }
        }
        dp[n] as _
    }

    fn square_numbers(limit: usize) -> Vec<usize> {
        let mut res = vec![];
        let mut i = 1;
        while i * i <= limit {
            res.push(i * i);
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 12;
        assert_eq!(Solution::num_squares(n), 3);

        let n = 13;
        assert_eq!(Solution::num_squares(n), 2);
    }
}
