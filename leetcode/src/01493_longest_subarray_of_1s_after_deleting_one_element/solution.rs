struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut zero_cnt = 0;
        let mut left = 0;
        let mut res = 0;
        for right in 0..nums.len() {
            if nums[right] == 0 {
                zero_cnt += 1;
            }
            while zero_cnt > 1 {
                if nums[left] == 0 {
                    zero_cnt -= 1;
                }
                left += 1;
            }
            res = res.max(right - left);
        }
        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 1, 0, 1];
        assert_eq!(Solution::longest_subarray(nums), 3);

        let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        assert_eq!(Solution::longest_subarray(nums), 5);
    }
}
