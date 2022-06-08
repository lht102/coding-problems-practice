struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (_, _, _, res) = Solution::divide_and_conquer(&nums, 0, nums.len() - 1);
        res
    }

    fn divide_and_conquer(nums: &[i32], left: usize, right: usize) -> (i32, i32, i32, i32) {
        if left == right {
            return (nums[left], nums[left], nums[left], nums[left]);
        }
        let mid = left + (right - left) / 2;
        let (left_sum_start, left_sum_end, left_sum_total, left_sum_max) =
            Solution::divide_and_conquer(nums, left, mid);
        let (right_sum_start, right_sum_end, right_sum_total, right_sum_max) =
            Solution::divide_and_conquer(nums, mid + 1, right);
        let sum1 = left_sum_start.max(left_sum_total + right_sum_start);
        let sum2 = right_sum_end.max(right_sum_total + left_sum_end);
        let sum3 = left_sum_max
            .max(right_sum_max)
            .max(left_sum_end + right_sum_start);
        (sum1, sum2, left_sum_total + right_sum_total, sum3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(nums), 6);

        let nums = vec![1];
        assert_eq!(Solution::max_sub_array(nums), 1);

        let nums = vec![5, 4, -1, 7, 8];
        assert_eq!(Solution::max_sub_array(nums), 23);

        let nums = vec![1, 2, -1, -2, 2, 1, -2, 1];
        assert_eq!(Solution::max_sub_array(nums), 3);
    }
}
