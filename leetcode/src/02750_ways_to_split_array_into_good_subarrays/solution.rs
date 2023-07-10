struct Solution;

impl Solution {
    pub fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {
        if let Some(idx) = nums.iter().position(|&x| x == 1) {
            let mut zero_cnt = 0;
            let mut res = 1i64;
            for &num in nums.iter().skip(idx) {
                if num == 1 {
                    res = res * (zero_cnt + 1) % 1_000_000_007;
                    zero_cnt = 0;
                } else {
                    zero_cnt += 1;
                }
            }
            res as _
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![0, 1, 0, 0, 1];
        assert_eq!(Solution::number_of_good_subarray_splits(nums), 3);

        let nums = vec![0, 1, 0];
        assert_eq!(Solution::number_of_good_subarray_splits(nums), 1);

        let nums = vec![1, 0, 0, 0, 0, 0, 1, 0, 1];
        assert_eq!(Solution::number_of_good_subarray_splits(nums), 12);

        let nums = vec![0, 1, 0, 0, 1];
        assert_eq!(Solution::number_of_good_subarray_splits(nums), 3);
    }
}
