struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut arr = nums.clone();
        arr.sort_unstable();
        let (mut lo, mut hi) = (nums.len(), 0);
        for i in 0..nums.len() {
            if arr[i] != nums[i] {
                lo = lo.min(i);
                hi = hi.max(i);
            }
        }
        if hi > lo { (hi - lo + 1) as i32 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 6, 4, 8, 10, 9, 15];
        assert_eq!(Solution::find_unsorted_subarray(nums), 5);

        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_unsorted_subarray(nums), 0);

        let nums = vec![1];
        assert_eq!(Solution::find_unsorted_subarray(nums), 0);

        let nums = vec![1, 3, 2, 2, 2, 4];
        assert_eq!(Solution::find_unsorted_subarray(nums), 4);

        let nums = vec![1, 2, 3, 3, 3];
        assert_eq!(Solution::find_unsorted_subarray(nums), 0);

        let nums = vec![1, 3, 2, 3, 3];
        assert_eq!(Solution::find_unsorted_subarray(nums), 2);

        let nums = vec![2, 3, 3, 2, 4];
        assert_eq!(Solution::find_unsorted_subarray(nums), 3);

        let nums = vec![1, 2, 4, 5, 3];
        assert_eq!(Solution::find_unsorted_subarray(nums), 3);
    }
}
