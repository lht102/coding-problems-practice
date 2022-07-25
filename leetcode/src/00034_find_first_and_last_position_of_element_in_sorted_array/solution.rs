struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut first = nums.partition_point(|&x| x < target) as i32;
        if first == n || nums[first as usize] != target {
            first = -1;
        }
        let mut last = nums.partition_point(|&x| x <= target) as i32 - 1;
        if last < 0 || nums[last as usize] != target {
            last = -1;
        }
        vec![first, last]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        assert_eq!(Solution::search_range(nums, target), vec![3, 4]);

        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        assert_eq!(Solution::search_range(nums, target), vec![-1, -1]);

        let nums = vec![];
        let target = 0;
        assert_eq!(Solution::search_range(nums, target), vec![-1, -1]);
    }
}
