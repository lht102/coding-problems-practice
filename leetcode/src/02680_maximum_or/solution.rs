use std::iter;

struct Solution;

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let pre = iter::once(0)
            .chain(nums.iter().scan(0, |acc, &x| {
                *acc |= x as i64;
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        let suf = iter::once(0)
            .chain(nums.iter().rev().scan(0, |acc, &x| {
                *acc |= x as i64;
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        (0..nums.len())
            .map(|i| pre[i] | (nums[i] as i64 * (1 << k)) | suf[nums.len() - i - 1])
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![12, 9];
        let k = 1;
        assert_eq!(Solution::maximum_or(nums, k), 30);

        let nums = vec![8, 1, 2];
        let k = 2;
        assert_eq!(Solution::maximum_or(nums, k), 35);
    }
}
