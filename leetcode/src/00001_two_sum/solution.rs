use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = hm.get(&(target - num)) {
                return vec![j, i as i32];
            } else {
                hm.insert(num, i as i32);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);

        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);

        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
