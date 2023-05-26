struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1.0;
        }
        if n >= k - 1 + max_pts {
            return 1.0;
        }
        let n = n as usize;
        let k = k as usize;
        let max_pts = max_pts as usize;

        let mut dp = vec![0.0; n + 1];
        dp[0] = 1.0;
        let mut window_sum = 1.0;

        let mut res = 0.0;
        for i in 1..=n {
            dp[i] = window_sum / max_pts as f64;
            if i < k {
                window_sum += dp[i];
            } else {
                res += dp[i];
            }
            if i >= max_pts {
                window_sum -= dp[i - max_pts];
            }
        }
        (res * 100000.0).round() / 100000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 10;
        let k = 1;
        let max_pts = 10;
        assert_eq!(Solution::new21_game(n, k, max_pts), 1.0);

        let n = 6;
        let k = 1;
        let max_pts = 10;
        assert_eq!(Solution::new21_game(n, k, max_pts), 0.6);

        let n = 21;
        let k = 17;
        let max_pts = 10;
        assert_eq!(Solution::new21_game(n, k, max_pts), 0.73278);
    }
}
