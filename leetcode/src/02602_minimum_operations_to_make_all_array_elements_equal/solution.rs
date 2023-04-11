struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        nums.sort_unstable();
        let psum = std::iter::once(0)
            .chain(nums.iter().scan(0, |sum, num| {
                *sum += num;
                Some(*sum)
            }))
            .collect::<Vec<_>>();
        queries
            .into_iter()
            .map(|q| {
                let q = q as i64;
                let idx = nums.partition_point(|&x| x < q);
                (q * idx as i64 - psum[idx])
                    + (*psum.last().unwrap() - psum[idx] - q * (nums.len() - idx) as i64)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 1, 6, 8];
        let queries = vec![1, 5];
        assert_eq!(Solution::min_operations(nums, queries), vec![14, 10]);

        let nums = vec![2, 9, 6, 3];
        let queries = vec![10];
        assert_eq!(Solution::min_operations(nums, queries), vec![20]);
    }
}
