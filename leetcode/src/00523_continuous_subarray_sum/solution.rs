use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut psum_to_idx = HashMap::from([(0, -1)]);
        let mut sum = 0;
        for (i, num) in nums.iter().enumerate() {
            sum += num;
            if let Some(idx) = psum_to_idx.get(&(sum % k)) {
                if i as i32 - idx > 1 {
                    return true;
                }
            } else {
                psum_to_idx.insert(sum % k, i as i32);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![23, 2, 4, 6, 7];
        let k = 6;
        assert!(Solution::check_subarray_sum(nums, k));

        let nums = vec![23, 2, 6, 4, 7];
        let k = 13;
        assert!(!Solution::check_subarray_sum(nums, k));

        let nums = vec![5, 0, 0, 0];
        let k = 3;
        assert!(Solution::check_subarray_sum(nums, k));
    }
}
