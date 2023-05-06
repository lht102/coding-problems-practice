use std::iter;

struct Solution;

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        const M: i32 = 1_000_000_007;
        let mut nums = nums;
        nums.sort_unstable();
        let pows = iter::once(1)
            .chain((1..nums.len()).scan(1, |acc, _| {
                *acc = *acc * 2 % M;
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        let mut res = 0;
        for (i, num) in nums.iter().enumerate() {
            let k = target - num;
            let j = nums.partition_point(|&x| x <= k);
            if j == 0 {
                continue;
            }
            if i < j {
                res = (res + pows[j - 1 - i]) % M;
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
        let nums = vec![3, 5, 6, 7];
        let target = 9;
        assert_eq!(Solution::num_subseq(nums, target), 4);

        let nums = vec![3, 3, 6, 8];
        let target = 10;
        assert_eq!(Solution::num_subseq(nums, target), 6);

        let nums = vec![2, 3, 3, 4, 6, 7];
        let target = 12;
        assert_eq!(Solution::num_subseq(nums, target), 61);
    }
}
