struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut farthest = 0;
        let mut cur_pos = 0;
        for (i, &num) in nums.iter().enumerate().take(nums.len() - 1) {
            farthest = farthest.max(num as usize + i);
            if i == cur_pos {
                cur_pos = farthest;
                res += 1;
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
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(Solution::jump(nums), 2);

        let nums = vec![2, 3, 0, 1, 4];
        assert_eq!(Solution::jump(nums), 2);
    }
}
