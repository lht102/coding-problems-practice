struct Solution;

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        if nums.len() < 5 {
            return 0;
        }

        (0..4)
            .map(|i| nums[nums.len() - 4 + i] - nums[i])
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![5, 3, 2, 4];
        assert_eq!(Solution::min_difference(nums), 0);

        let nums = vec![1, 5, 0, 10, 14];
        assert_eq!(Solution::min_difference(nums), 1);
    }
}
