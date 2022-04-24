struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = (hi - lo) / 2 + lo;
            if nums[mid] < nums[hi] {
                hi = mid
            } else {
                lo = mid + 1
            }
        }

        if target == nums[lo] {
            return lo as i32;
        }
        let start = if target <= *nums.last().unwrap() {
            lo
        } else {
            0
        };
        let end = if target > *nums.last().unwrap() {
            lo
        } else {
            nums.len()
        };
        match nums[start..end].binary_search(&target) {
            Ok(res) => (res + start) as i32,
            Err(_) => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(Solution::search(nums, target), 4);

        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(Solution::search(nums, target), -1);

        let nums = vec![1];
        let target = 0;
        assert_eq!(Solution::search(nums, target), -1);

        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 2;
        assert_eq!(Solution::search(nums, target), 6);

        let nums = vec![3, 1];
        let target = 3;
        assert_eq!(Solution::search(nums, target), 0);
    }
}
