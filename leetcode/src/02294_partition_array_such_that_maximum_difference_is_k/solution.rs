struct Solution;

impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut res = 1;
        let mut j = 0;
        for (i, &num) in nums.iter().enumerate().skip(1) {
            if num - nums[j] > k {
                res += 1;
                j = i;
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
        let nums = vec![3, 6, 1, 2, 5];
        let k = 2;
        assert_eq!(Solution::partition_array(nums, k), 2);

        let nums = vec![1, 2, 3];
        let k = 1;
        assert_eq!(Solution::partition_array(nums, k), 2);
    }
}
