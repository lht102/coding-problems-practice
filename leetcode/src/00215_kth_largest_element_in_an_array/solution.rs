struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        Solution::quick_select(&mut nums, 0, n - 1, n - k as usize)
    }

    fn quick_select(nums: &mut Vec<i32>, start: usize, end: usize, rank: usize) -> i32 {
        if start == end {
            return nums[start];
        }
        let p = Solution::partition(nums, start, end);
        if p < rank {
            Solution::quick_select(nums, p + 1, end, rank)
        } else if p > rank {
            Solution::quick_select(nums, start, p - 1, rank)
        } else {
            nums[p]
        }
    }

    fn partition(nums: &mut Vec<i32>, start: usize, end: usize) -> usize {
        let pivot = nums[end];
        let mut left = start;
        for i in start..end {
            if nums[i] <= pivot {
                nums.swap(left, i);
                left += 1;
            }
        }
        nums.swap(left, end);
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        assert_eq!(Solution::find_kth_largest(nums, k), 5);

        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        assert_eq!(Solution::find_kth_largest(nums, k), 4);
    }
}
