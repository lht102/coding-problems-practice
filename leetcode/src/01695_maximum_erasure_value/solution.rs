use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut sum = 0;
        let mut seen = HashSet::new();
        let n = nums.len();
        let (mut start, mut end) = (0, 0);
        while end < n {
            if seen.contains(&nums[end]) {
                sum -= nums[start];
                seen.remove(&nums[start]);
                start += 1;
            } else {
                sum += nums[end];
                seen.insert(nums[end]);
                end += 1;
                res = res.max(sum);
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
        let nums = vec![4, 2, 4, 5, 6];
        assert_eq!(Solution::maximum_unique_subarray(nums), 17);

        let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        assert_eq!(Solution::maximum_unique_subarray(nums), 8);
    }
}
