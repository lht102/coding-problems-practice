use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut closest_sum = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let local_sum = nums[i] + nums[left] + nums[right];
                if (local_sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = local_sum;
                }
                match local_sum.cmp(&target) {
                    Ordering::Less => left += 1,
                    Ordering::Equal => return target,
                    Ordering::Greater => right -= 1,
                }
            }
        }
        closest_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        assert_eq!(Solution::three_sum_closest(nums, target), 2);

        let nums = vec![0, 0, 0];
        let target = 1;
        assert_eq!(Solution::three_sum_closest(nums, target), 0);
    }
}
