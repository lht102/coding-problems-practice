struct Solution;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = nums1.len();
        let n = nums2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut res = 0;
        for i in 1..=m {
            for j in 1..=n {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    res = res.max(dp[i][j]);
                }
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
        let nums1 = vec![1, 2, 3, 2, 1];
        let nums2 = vec![3, 2, 1, 4, 7];
        assert_eq!(Solution::find_length(nums1, nums2), 3);

        let nums1 = vec![0, 0, 0, 0, 0];
        let nums2 = vec![0, 0, 0, 0, 0];
        assert_eq!(Solution::find_length(nums1, nums2), 5);
    }
}
