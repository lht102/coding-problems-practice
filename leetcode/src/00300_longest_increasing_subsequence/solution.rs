struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = Vec::new();
        for num in nums {
            let i = dp.partition_point(|&x| x < num);
            if i == dp.len() {
                dp.push(num);
            } else {
                dp[i] = num;
            }
        }
        dp.len() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(Solution::length_of_lis(nums), 4);

        let nums = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(Solution::length_of_lis(nums), 4);

        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(Solution::length_of_lis(nums), 1);
    }
}
