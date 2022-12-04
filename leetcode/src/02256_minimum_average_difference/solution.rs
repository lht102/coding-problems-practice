struct Solution;

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let psum = nums
            .iter()
            .scan(0i64, |acc, &num| {
                *acc += num as i64;
                Some(*acc)
            })
            .collect::<Vec<_>>();
        (0..psum.len())
            .map(|i| (Self::difference(&psum, i), i))
            .min()
            .unwrap()
            .1 as i32
    }

    fn difference(psum: &[i64], idx: usize) -> i64 {
        if idx == psum.len() - 1 {
            return psum[psum.len() - 1] / psum.len() as i64;
        }
        (psum[idx] / (idx + 1) as i64
            - (psum[psum.len() - 1] - psum[idx]) / (psum.len() - 1 - idx) as i64)
            .abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 5, 3, 9, 5, 3];
        assert_eq!(Solution::minimum_average_difference(nums), 3);

        let nums = vec![0];
        assert_eq!(Solution::minimum_average_difference(nums), 0);
    }
}
