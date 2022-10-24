use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::<i32, usize>::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(j) = map.get(&num) {
                if i - j <= k as _ {
                    return true;
                }
            }
            map.insert(num, i);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        assert!(Solution::contains_nearby_duplicate(nums, k));

        let nums = vec![1, 0, 1, 1];
        let k = 1;
        assert!(Solution::contains_nearby_duplicate(nums, k));

        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        assert!(!Solution::contains_nearby_duplicate(nums, k));
    }
}
