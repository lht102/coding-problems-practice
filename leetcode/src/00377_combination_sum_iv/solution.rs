struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![-1; target as usize + 1];
        Solution::solve(&nums, target, &mut dp)
    }

    fn solve(nums: &[i32], target: i32, dp: &mut [i32]) -> i32 {
        if target == 0 {
            return 1;
        }
        let t = target as usize;
        if dp[t] != -1 {
            return dp[t];
        }
        dp[t] = nums.iter().fold(0, |res, &num| {
            if num <= target {
                res + Solution::solve(nums, target - num, dp)
            } else {
                res
            }
        });
        dp[t]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3];
        let target = 4;
        assert_eq!(Solution::combination_sum4(nums, target), 7);

        let nums = vec![9];
        let target = 3;
        assert_eq!(Solution::combination_sum4(nums, target), 0);
    }
}
