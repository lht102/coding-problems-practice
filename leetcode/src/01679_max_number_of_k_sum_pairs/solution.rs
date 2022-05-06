use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut freq = HashMap::<i32, usize>::new();
        for &num in &nums {
            let diff = k - num;
            match freq.get_mut(&diff) {
                Some(x) if *x > 0 => {
                    *x -= 1;
                    res += 1;
                }
                _ => {
                    *freq.entry(num).or_default() += 1;
                }
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
        let nums = vec![1, 2, 3, 4];
        let k = 5;
        assert_eq!(Solution::max_operations(nums, k), 2);

        let nums = vec![3, 1, 3, 4, 3];
        let k = 6;
        assert_eq!(Solution::max_operations(nums, k), 1);

        let nums = vec![4, 4, 1, 3, 1, 3, 2, 2, 5, 5, 1, 5, 2, 1, 2, 3, 5, 4];
        let k = 2;
        assert_eq!(Solution::max_operations(nums, k), 2);
    }
}
