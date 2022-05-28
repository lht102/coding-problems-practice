struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut res = nums.len() as i32;
        for i in 0..nums.len() {
            res ^= i as i32 ^ nums[i];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 0, 1];
        assert_eq!(Solution::missing_number(nums), 2);

        let nums = vec![0, 1];
        assert_eq!(Solution::missing_number(nums), 2);

        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(Solution::missing_number(nums), 8);
    }
}
