struct Solution;

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut cnt = 0;
        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                cnt += 1;
                if cnt > 1
                    || i > 1
                        && i + 1 < nums.len()
                        && nums[i - 2] > nums[i]
                        && nums[i - 1] > nums[i + 1]
                {
                    return false;
                }
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
        let nums = vec![4, 2, 3];
        assert!(Solution::check_possibility(nums));

        let nums = vec![4, 2, 1];
        assert!(!Solution::check_possibility(nums));
    }
}
