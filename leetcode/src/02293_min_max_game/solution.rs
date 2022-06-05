struct Solution;

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut new_nums = vec![0; nums.len() / 2];
        for i in 0..nums.len() / 2 {
            if i & 1 == 0 {
                new_nums[i] = nums[2 * i].min(nums[2 * i + 1]);
            } else {
                new_nums[i] = nums[2 * i].max(nums[2 * i + 1]);
            }
        }
        Solution::min_max_game(new_nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 3, 5, 2, 4, 8, 2, 2];
        assert_eq!(Solution::min_max_game(nums), 1);

        let nums = vec![3];
        assert_eq!(Solution::min_max_game(nums), 3);

        let nums = vec![70, 38, 21, 22];
        assert_eq!(Solution::min_max_game(nums), 22);
    }
}
