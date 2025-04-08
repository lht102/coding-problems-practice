use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let psum = std::iter::once(0)
            .chain(nums.iter().scan(0, |acc, num| {
                *acc += num;
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        let psum_to_idx = psum.iter().enumerate().fold(
            HashMap::<i32, usize>::with_capacity(psum.len()),
            |mut map, (i, &num)| {
                *map.entry(num).or_default() = i;
                map
            },
        );
        let ssum = std::iter::once(0)
            .chain(nums.iter().rev().scan(0, |acc, num| {
                *acc += num;
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        let mut res = usize::MAX;
        for (end, sum) in ssum.iter().enumerate() {
            if let Some(&start) = psum_to_idx.get(&(x - sum)) {
                if start + end <= nums.len() {
                    res = res.min(start + end);
                }
            }
        }
        if res == usize::MAX { -1 } else { res as i32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 1, 4, 2, 3];
        let x = 5;
        assert_eq!(Solution::min_operations(nums, x), 2);

        let nums = vec![5, 6, 7, 8, 9];
        let x = 4;
        assert_eq!(Solution::min_operations(nums, x), -1);

        let nums = vec![3, 2, 30, 1, 1, 3];
        let x = 10;
        assert_eq!(Solution::min_operations(nums, x), 5);

        let nums = vec![1, 1];
        let x = 3;
        assert_eq!(Solution::min_operations(nums, x), -1);

        let nums = vec![1, 1];
        let x = 2;
        assert_eq!(Solution::min_operations(nums, x), 2);

        let nums = vec![
            8828, 9581, 49, 9818, 9974, 9869, 9991, 10000, 10000, 10000, 9999, 9993, 9904, 8819,
            1231, 6309,
        ];
        let x = 134365;
        assert_eq!(Solution::min_operations(nums, x), 16);

        let nums = vec![5, 2, 3, 1, 1];
        let x = 5;
        assert_eq!(Solution::min_operations(nums, x), 1);
    }
}
