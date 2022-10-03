struct Solution;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; target as usize + 1]; n as usize + 1];
        Solution::solve(n, k, target, &mut dp)
    }

    fn solve(n: i32, k: i32, target: i32, dp: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if n == 0 && target == 0 {
            return 1;
        }
        if let Some(val) = dp[n as usize][target as usize] {
            return val;
        }
        let mut res = 0;
        if n > 0 {
            for i in 1..=k {
                if target >= i {
                    res = (res + Solution::solve(n - 1, k, target - i, dp)) % 1_000_000_007;
                }
            }
        }
        dp[n as usize][target as usize] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 1;
        let k = 6;
        let target = 3;
        assert_eq!(Solution::num_rolls_to_target(n, k, target), 1);

        let n = 2;
        let k = 6;
        let target = 7;
        assert_eq!(Solution::num_rolls_to_target(n, k, target), 6);

        let n = 30;
        let k = 30;
        let target = 500;
        assert_eq!(Solution::num_rolls_to_target(n, k, target), 222616187);
    }
}
