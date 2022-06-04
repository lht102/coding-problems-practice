struct Solution;

impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        if 6 * nums1.len() < nums2.len() || nums1.len() > 6 * nums2.len() {
            return -1;
        }
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        let mut sum1: i32 = nums1.iter().sum();
        let mut sum2: i32 = nums2.iter().sum();
        if sum1 == sum2 {
            return 0;
        }
        if sum1 > sum2 {
            std::mem::swap(&mut sum1, &mut sum2);
            std::mem::swap(&mut nums1, &mut nums2);
        }
        let mut freq = vec![0; 6];
        for &num in &nums1 {
            freq[6 - num as usize] += 1;
        }
        for &num in &nums2 {
            freq[num as usize - 1] += 1;
        }
        let mut res = 0;
        for i in (1..6).rev() {
            while freq[i] > 0 {
                sum1 += i as i32;
                res += 1;
                if sum1 >= sum2 {
                    return res;
                }
                freq[i] -= 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1 = vec![1, 2, 3, 4, 5, 6];
        let nums2 = vec![1, 1, 2, 2, 2, 2];
        assert_eq!(Solution::min_operations(nums1, nums2), 3);

        let nums1 = vec![1, 1, 1, 1, 1, 1, 1];
        let nums2 = vec![6];
        assert_eq!(Solution::min_operations(nums1, nums2), -1);

        let nums1 = vec![6, 6];
        let nums2 = vec![1];
        assert_eq!(Solution::min_operations(nums1, nums2), 3);
    }
}
