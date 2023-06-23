use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut dp = vec![HashMap::new(); n];
        for right in 0..n {
            for left in 0..right {
                let diff = nums[left] - nums[right];
                let end_len = *dp[left].entry(diff).or_insert(1) + 1;
                dp[right].insert(diff, end_len);
                res = res.max(end_len);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 6, 9, 12];
        assert_eq!(Solution::longest_arith_seq_length(nums), 4);

        let nums = vec![9, 4, 7, 2, 10];
        assert_eq!(Solution::longest_arith_seq_length(nums), 3);

        let nums = vec![20, 1, 15, 3, 10, 5, 8];
        assert_eq!(Solution::longest_arith_seq_length(nums), 4);

        let nums = vec![
            44, 46, 22, 68, 45, 66, 43, 9, 37, 30, 50, 67, 32, 47, 44, 11, 15, 4, 11, 6, 20, 64,
            54, 54, 61, 63, 23, 43, 3, 12, 51, 61, 16, 57, 14, 12, 55, 17, 18, 25, 19, 28, 45, 56,
            29, 39, 52, 8, 1, 21, 17, 21, 23, 70, 51, 61, 21, 52, 25, 28,
        ];
        assert_eq!(Solution::longest_arith_seq_length(nums), 6);
    }
}
