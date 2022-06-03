struct Solution;

impl Solution {
    pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            return nums[0];
        }
        if nums.len() == 1 && k % 2 == 1 {
            return -1;
        }
        let k = k as usize;
        let mut res = -1;
        for &num in nums.iter().take(nums.len().min(k - 1)) {
            res = res.max(num);
        }
        if k < nums.len() {
            res = res.max(nums[k]);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![5, 2, 2, 4, 0, 6];
        let k = 4;
        assert_eq!(Solution::maximum_top(nums, k), 5);

        let nums = vec![2];
        let k = 1;
        assert_eq!(Solution::maximum_top(nums, k), -1);

        let nums = vec![1, 2];
        let k = 2;
        assert_eq!(Solution::maximum_top(nums, k), 1);

        let nums = vec![
            91, 98, 17, 79, 15, 55, 47, 86, 4, 5, 17, 79, 68, 60, 60, 31, 72, 85, 25, 77, 8, 78,
            40, 96, 76, 69, 95, 2, 42, 87, 48, 72, 45, 25, 40, 60, 21, 91, 32, 79, 2, 87, 80, 97,
            82, 94, 69, 43, 18, 19, 21, 36, 44, 81, 99,
        ];
        let k = 2;
        assert_eq!(Solution::maximum_top(nums, k), 91);
    }
}
