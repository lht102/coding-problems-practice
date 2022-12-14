struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return nums[0];
        }
        let mut dp = vec![0; n + 1];
        dp[1] = nums[0];
        dp[2] = nums[0].max(nums[1]);
        for i in 2..=n {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i - 1]);
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(nums), 4);

        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(nums), 12);
    }
}
