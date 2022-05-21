use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut psum_to_idx = HashMap::from([(0, -1)]);
        let mut cnt = 0;
        for (i, &num) in nums.iter().enumerate() {
            cnt += if num == 0 { -1 } else { 1 };
            if let Some(idx) = psum_to_idx.get(&cnt) {
                res = res.max(i as i32 - idx);
            } else {
                psum_to_idx.insert(cnt, i as i32);
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
        let nums = vec![0, 1];
        assert_eq!(Solution::find_max_length(nums), 2);

        let nums = vec![0, 1, 0];
        assert_eq!(Solution::find_max_length(nums), 2);

        let nums = vec![0, 1, 0, 1];
        assert_eq!(Solution::find_max_length(nums), 4);
    }
}
