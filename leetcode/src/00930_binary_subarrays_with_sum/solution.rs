use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut psum = HashMap::from([(0, 1)]);
        let mut sum = 0;
        let mut res = 0;
        for &num in &nums {
            sum += num;
            if let Some(cnt) = psum.get(&(sum - goal)) {
                res += cnt;
            }
            *psum.entry(sum).or_default() += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 0, 1, 0, 1];
        let goal = 2;
        assert_eq!(Solution::num_subarrays_with_sum(nums, goal), 4);

        let nums = vec![0, 0, 0, 0, 0];
        let goal = 0;
        assert_eq!(Solution::num_subarrays_with_sum(nums, goal), 15);
    }
}
