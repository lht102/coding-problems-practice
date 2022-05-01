struct Solution;

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix = nums.into_iter().map(|num| num as i64).collect::<Vec<_>>();
        for i in 1..n {
            prefix[i] += prefix[i - 1];
        }
        (0..n)
            .map(|i| (Self::difference(&prefix, i), i))
            .min()
            .unwrap()
            .1 as i32
    }

    fn difference(prefix: &[i64], idx: usize) -> i64 {
        if idx == prefix.len() - 1 {
            return prefix[prefix.len() - 1] / prefix.len() as i64;
        }
        (prefix[idx] / (idx + 1) as i64
            - (prefix[prefix.len() - 1] - prefix[idx]) / (prefix.len() - 1 - idx) as i64)
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
