struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len());
        while lo < hi {
            let mid = (hi - lo) / 2 + lo;
            if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(Solution::search_insert(nums, target), 2);

        let nums = vec![1, 3, 5, 6];
        let target = 2;
        assert_eq!(Solution::search_insert(nums, target), 1);

        let nums = vec![1, 3, 5, 6];
        let target = 7;
        assert_eq!(Solution::search_insert(nums, target), 4);
    }
}
