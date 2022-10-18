struct Solution;

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (nums[0], *nums.iter().max().unwrap());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if Solution::is_valid(&nums, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    fn is_valid(nums: &[i32], max_num: i32) -> bool {
        let mut sum: i64 = 0;
        for (i, &num) in nums.iter().enumerate() {
            sum += num as i64;
            if sum > max_num as i64 * (i + 1) as i64 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 7, 1, 6];
        assert_eq!(Solution::minimize_array_value(nums), 5);

        let nums = vec![10, 1];
        assert_eq!(Solution::minimize_array_value(nums), 10);

        let nums = vec![13, 13, 20, 0, 8, 9, 9];
        assert_eq!(Solution::minimize_array_value(nums), 16);
    }
}
