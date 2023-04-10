struct Solution;

impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let (mut lo, mut hi) = (0, nums.last().unwrap() - nums.first().unwrap());
        while lo < hi {
            let mid = (hi - lo) / 2 + lo;
            if Self::is_valid(&nums, p, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    fn is_valid(nums: &[i32], p: i32, diff: i32) -> bool {
        let mut i = 1;
        let mut cnt = 0;
        while i < nums.len() {
            if nums[i] - nums[i - 1] <= diff {
                cnt += 1;
                i += 1;
            }
            i += 1;
        }
        cnt >= p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![10, 1, 2, 7, 1, 3];
        let p = 2;
        assert_eq!(Solution::minimize_max(nums, p), 1);

        let nums = vec![4, 2, 1, 2];
        let p = 1;
        assert_eq!(Solution::minimize_max(nums, p), 0);
    }
}
