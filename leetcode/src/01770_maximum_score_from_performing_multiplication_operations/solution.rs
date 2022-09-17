struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut dp = vec![0; m + 1];
        for op in (0..m).rev() {
            for left in 0..=op {
                dp[left] = (multipliers[op] * nums[left] + dp[left + 1])
                    .max(multipliers[op] * nums[n - 1 - op + left] + dp[left]);
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3];
        let multipliers = vec![3, 2, 1];
        assert_eq!(Solution::maximum_score(nums, multipliers), 14);

        let nums = vec![-5, -3, -3, -2, 7, 1];
        let multipliers = vec![-10, -5, 3, 4, 6];
        assert_eq!(Solution::maximum_score(nums, multipliers), 102);
    }
}
