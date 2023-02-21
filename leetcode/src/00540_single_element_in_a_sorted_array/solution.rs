struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = (hi - lo) / 2 + lo;
            if nums[mid] == nums[mid ^ 1] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        nums[lo]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
        assert_eq!(Solution::single_non_duplicate(nums), 2);

        let nums = vec![3, 3, 7, 7, 10, 11, 11];
        assert_eq!(Solution::single_non_duplicate(nums), 10);

        let nums = vec![1, 1, 2];
        assert_eq!(Solution::single_non_duplicate(nums), 2);

        let nums = vec![1];
        assert_eq!(Solution::single_non_duplicate(nums), 1);
    }
}
