use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        Self::dfs(&nums, k, 0, &mut HashMap::new()) - 1
    }

    fn dfs(nums: &[i32], k: i32, i: usize, freq: &mut HashMap<i32, usize>) -> i32 {
        if i == nums.len() {
            return 1;
        }
        let mut res = Self::dfs(nums, k, i + 1, freq);
        if *freq.get(&(nums[i] - k)).unwrap_or(&0) == 0 {
            *freq.entry(nums[i]).or_default() += 1;
            res += Self::dfs(nums, k, i + 1, freq);
            *freq.entry(nums[i]).or_default() -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 4, 6];
        let k = 2;
        assert_eq!(Solution::beautiful_subsets(nums, k), 4);

        let nums = vec![1];
        let k = 1;
        assert_eq!(Solution::beautiful_subsets(nums, k), 1);
    }
}
