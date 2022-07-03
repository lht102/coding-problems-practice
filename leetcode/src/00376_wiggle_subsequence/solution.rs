struct Solution;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![-1; 2]; n];
        Solution::solve(&nums, 1, n - 1, &mut dp).max(Solution::solve(&nums, -1, n - 1, &mut dp))
    }

    fn solve(nums: &[i32], sign: i32, idx: usize, dp: &mut [Vec<i32>]) -> i32 {
        if idx == 0 {
            return 1;
        }
        let j = (if sign == -1 { 0 } else { 1 }) as usize;
        if dp[idx][j] != -1 {
            return dp[idx][j];
        }
        dp[idx][j] = if (nums[idx] - nums[idx - 1]) * sign < 0 {
            Solution::solve(nums, -sign, idx - 1, dp) + 1
        } else {
            Solution::solve(nums, sign, idx - 1, dp)
        };
        dp[idx][j]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 7, 4, 9, 2, 5];
        assert_eq!(Solution::wiggle_max_length(nums), 6);

        let nums = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
        assert_eq!(Solution::wiggle_max_length(nums), 7);

        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Solution::wiggle_max_length(nums), 2);
    }
}
