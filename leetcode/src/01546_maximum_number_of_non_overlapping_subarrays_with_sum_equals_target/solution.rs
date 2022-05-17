use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let mut psum_to_idx = HashMap::from([(0, -1)]);
        let mut res = 0;
        let mut idx = -1;
        let mut sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            sum += num;
            if let Entry::Occupied(e) = psum_to_idx.entry(sum - target) {
                if *e.get() >= idx {
                    res += 1;
                    idx = i as i32;
                }
            }
            *psum_to_idx.entry(sum).or_default() = i as i32;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 1, 1, 1, 1];
        let target = 2;
        assert_eq!(Solution::max_non_overlapping(nums, target), 2);

        let nums = vec![-1, 3, 5, 1, 4, 2, -9];
        let target = 6;
        assert_eq!(Solution::max_non_overlapping(nums, target), 2);
    }
}
