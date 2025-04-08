struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let psum = std::iter::once(0)
            .chain(nums.iter().scan(0, |sum, num| {
                *sum += num;
                Some(*sum)
            }))
            .collect::<Vec<_>>();
        let mut res = usize::MAX;
        for (i, cur_sum) in psum.iter().enumerate() {
            let k = cur_sum - target;
            if k < 0 {
                continue;
            }
            let j = psum.partition_point(|&x| x <= k);
            res = res.min(i - j + 1);
        }
        if res == usize::MAX { 0 } else { res as i32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        assert_eq!(Solution::min_sub_array_len(target, nums), 2);

        let target = 4;
        let nums = vec![1, 4, 4];
        assert_eq!(Solution::min_sub_array_len(target, nums), 1);

        let target = 11;
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(Solution::min_sub_array_len(target, nums), 0);

        let target = 1;
        let nums = vec![1];
        assert_eq!(Solution::min_sub_array_len(target, nums), 1);
    }
}
