struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut cnt = 0;
        for num in nums {
            if num == 0 {
                cnt += 1;
            } else {
                cnt = 0;
            }
            res += cnt;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 3, 0, 0, 2, 0, 0, 4];
        assert_eq!(Solution::zero_filled_subarray(nums), 6);

        let nums = vec![0, 0, 0, 2, 0, 0];
        assert_eq!(Solution::zero_filled_subarray(nums), 9);

        let nums = vec![2, 10, 2019];
        assert_eq!(Solution::zero_filled_subarray(nums), 0);
    }
}
