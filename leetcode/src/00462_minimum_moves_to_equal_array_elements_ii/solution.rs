struct Solution;

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut res = 0;
        while left < right {
            res += nums[right] - nums[left];
            left += 1;
            right -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::min_moves2(nums), 2);

        let nums = vec![1, 10, 2, 9];
        assert_eq!(Solution::min_moves2(nums), 16);
    }
}
