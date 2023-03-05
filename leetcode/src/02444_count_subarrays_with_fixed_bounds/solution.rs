struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut res = 0;
        let mut left_bound_idx = -1;
        let (mut min_val_idx, mut max_val_idx) = (-1, -1);
        for (i, &num) in nums.iter().enumerate() {
            if num < min_k || num > max_k {
                left_bound_idx = i as i32;
            }
            if num == min_k {
                min_val_idx = i as i32;
            }
            if num == max_k {
                max_val_idx = i as i32;
            }
            res += 0.max(min_val_idx.min(max_val_idx) - left_bound_idx) as i64;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 3, 5, 2, 7, 5];
        let min_k = 1;
        let max_k = 5;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 2);

        let nums = vec![1, 1, 1, 1];
        let min_k = 1;
        let max_k = 1;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 10);
    }
}
